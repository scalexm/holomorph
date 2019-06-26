use crate::session::{Session, State, AES_KEY_LEN, SALT_LEN};
use crate::RSA_KEY_SIZE;
use diesel::PgConnection;
use hashbrown::HashMap;
use log::{debug, error};
use protocol::messages::connection::IdentificationMessage;

#[derive(PartialEq, Eq, Debug, Queryable)]
struct Account {
    id: i32,
    login: String,
    nickname: String,
    last_server: Option<i16>,
}

struct Authenticated {
    account: Account,
    character_counts: HashMap<i16, u8>,
}

fn authenticate(
    conn: &PgConnection,
    login: &str,
    password: &str,
    ticket: &str,
) -> diesel::result::QueryResult<Authenticated> {
    use database::accounts::dsl;
    use database::characters::dsl as chars_dsl;
    use diesel::prelude::*;

    debug!("authenticating {}:{}", login, password);

    let account: Account = dsl::accounts
        .filter(dsl::login.eq(&login.to_lowercase()))
        // FIXME: we'll need to encrypt passwords in the future, of course.
        .filter(dsl::password.eq(password))
        .select((dsl::id, dsl::login, dsl::nickname, dsl::last_server))
        .first(conn)?;

    let query_result = chars_dsl::characters
        .filter(chars_dsl::account_id.eq(&account.id))
        .select((chars_dsl::server_id,))
        .load::<(i16,)>(conn)?;

    debug!("setting ticket to {}", ticket);

    diesel::update(dsl::accounts.find(account.id))
        .set(dsl::ticket.eq(ticket))
        .execute(conn)?;

    let mut character_counts: HashMap<i16, u8> = HashMap::new();
    for (server_id,) in query_result {
        let count = character_counts.entry(server_id).or_insert(0);
        *count = count.saturating_add(1);
    }

    Ok(Authenticated {
        account,
        character_counts,
    })
}

impl Session {
    pub async fn handle_identification<'a>(
        &'a mut self,
        msg: IdentificationMessage<'a>,
    ) -> std::io::Result<()> {
        use protocol::constants::identification_failure_reason;
        use protocol::constants::server_status;
        use protocol::messages::connection::IdentificationFailedMessage;
        use protocol::messages::connection::IdentificationSuccessMessage;
        use protocol::messages::connection::ServersListMessage;
        use protocol::types::connection::GameServerInformations;
        use rand::Rng;

        const TICKET_LEN: usize = 32;

        if self.state != State::Init {
            return Ok(());
        } else if msg.credentials.len() != RSA_KEY_SIZE as _ {
            debug!("wrong credentials len");
            return Ok(());
        }

        // Just convert from an `&[i8]` to an `&[u8]`.
        let credentials = unsafe {
            std::slice::from_raw_parts(msg.credentials.as_ptr() as *const u8, msg.credentials.len())
        };

        let ticket: String = {
            let mut rng = rand::thread_rng();
            std::iter::repeat(())
                .map(|()| rng.sample(rand::distributions::Alphanumeric))
                .take(TICKET_LEN)
                .collect()
        };

        debug!("moving to background thread");
        let server = &self.server;

        let fut = super::compat_poll_fn(|| {
            tokio_threadpool::blocking(|| {
                use openssl::rsa::Padding;

                const CERTIFICATE_LENGTH: usize = 32;

                let mut decrypted = [0; RSA_KEY_SIZE as _];
                let n = server
                    .private_key
                    .private_decrypt(&credentials, &mut decrypted, Padding::PKCS1)
                    .map_err(IdentificationError::DecryptionError)?;
                if n < SALT_LEN + AES_KEY_LEN {
                    return Err(IdentificationError::DecodeError);
                }

                // Just skip the salt, we don't use it.
                let mut decrypted = &decrypted[SALT_LEN..n];

                let mut aes_key = [0; AES_KEY_LEN];
                aes_key.copy_from_slice(&decrypted[..AES_KEY_LEN]);
                decrypted = &decrypted[AES_KEY_LEN..];

                if msg.use_certificate {
                    if decrypted.len() < CERTIFICATE_LENGTH {
                        return Err(IdentificationError::DecodeError);
                    }
                    decrypted = &decrypted[CERTIFICATE_LENGTH..];
                }

                let login_len = decrypted[0] as usize;
                if decrypted.len() < login_len + 1 {
                    return Err(IdentificationError::DecodeError);
                }
                let login = std::str::from_utf8(&decrypted[1..login_len + 1])
                    .map_err(|_| IdentificationError::DecodeError)?;
                let password = std::str::from_utf8(&decrypted[login_len + 1..])
                    .map_err(|_| IdentificationError::DecodeError)?;

                let conn = server
                    .database_pool
                    .get()
                    .map_err(IdentificationError::DatabasePoolError)?;
                let authenticated = authenticate(&conn, login, password, &ticket)
                    .map_err(IdentificationError::SqlError)?;

                Ok((authenticated, aes_key))
            })
        })
        .await
        .unwrap();

        let (authenticated, aes_key) = match fut {
            Ok(result) => result,
            Err(err) => {
                let reason = match err {
                    IdentificationError::SqlError(diesel::result::Error::NotFound) => {
                        identification_failure_reason::WRONG_CREDENTIALS
                    }

                    IdentificationError::DecodeError => {
                        debug!("decode error");
                        identification_failure_reason::UNKNOWN_AUTH_ERROR
                    }

                    IdentificationError::DatabasePoolError(err) => {
                        error!("could not get a connection from the database pool: {}", err);
                        identification_failure_reason::TIME_OUT
                    }

                    err => {
                        error!("unexpected error: {}", err);
                        identification_failure_reason::UNKNOWN_AUTH_ERROR
                    }
                };

                return self
                    .stream
                    .send(IdentificationFailedMessage {
                        reason,
                        _phantom: std::marker::PhantomData,
                    })
                    .await;
            }
        };

        self.stream.write(IdentificationSuccessMessage {
            account_creation: 0.,
            account_id: authenticated.account.id as u32,
            has_rights: false,
            havenbag_available_room: 0,
            login: &authenticated.account.login,
            community_id: 0,
            secret_question: "foo?",
            nickname: &authenticated.account.nickname,
            was_already_connected: false,
            subscription_elapsed_duration: 0.,
            subscription_end_date: 0.,
        })?;

        debug!("successfully logged in");

        self.state = State::Logged { aes_key, ticket };

        if msg.autoconnect {
            // Is `msg.autoconnect` really used? It seems that the client
            // just automatically sends a `ServerSelection` message when
            // auto-connect is enabled.
            if let Some(server_id) = authenticated.account.last_server {
                debug!("auto-connecting to game server");
                // `select_server` will take care of flushing the stream in
                // case of success.
                if self.select_server(server_id).await? == 0 {
                    return Ok(());
                }
            }
        }

        let game_server_information: Vec<_> = self
            .server
            .game_servers
            .values()
            .map(|gs| GameServerInformations {
                is_mono_account: false,
                is_selectable: gs.status() == server_status::ONLINE,
                id: gs.id() as _,
                type_: 0,
                status: gs.status(),
                completion: 0,
                characters_count: *authenticated.character_counts.get(&gs.id()).unwrap_or(&0),
                characters_slots: 5,
                date: 0.,
                _phantom: std::marker::PhantomData,
            })
            .collect();

        self.stream.write(ServersListMessage {
            servers: game_server_information.into(),
            already_connected_to_server_id: 0,
            can_create_new_character: true,
        })?;

        self.stream.flush().await
    }
}

enum IdentificationError {
    DecryptionError(openssl::error::ErrorStack),
    DecodeError,
    DatabasePoolError(diesel::r2d2::PoolError),
    SqlError(diesel::result::Error),
}

impl std::fmt::Display for IdentificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentificationError::DecryptionError(err) => write!(f, "{}", err),
            IdentificationError::DecodeError => write!(f, "decode error"),
            IdentificationError::DatabasePoolError(err) => write!(f, "{}", err),
            IdentificationError::SqlError(err) => write!(f, "{}", err),
        }
    }
}
