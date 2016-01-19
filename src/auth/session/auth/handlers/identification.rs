use std::io;
use protocol::{Protocol, VarShort, Flag};
use protocol::messages::connection::*;
use protocol::messages::queues::*;
use protocol::enums::{server_status, identification_failure_reason};
use session::auth::{AccountData, Session, QueueState};
use session::auth::chunk::{Ref, ChunkImpl};
use diesel::*;
use server::{self, SERVER};
use shared::{database, crypto};
use time;
use std::collections::HashMap;
use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};
use openssl::crypto::pkey::{EncryptionPadding, PKey};

pub static QUEUE_SIZE: AtomicIsize = ATOMIC_ISIZE_INIT;
pub static QUEUE_COUNTER: AtomicIsize = ATOMIC_ISIZE_INIT;

enum Error {
    Sql(result::Error),
    Reason(i8),
    Banned(i64),
}

impl From<result::Error> for Error {
    fn from(err: result::Error) -> Error {
        Error::Sql(err)
    }
}

impl From<result::TransactionError<Error>> for Error {
    fn from(err: result::TransactionError<Error>) -> Error {
        match err {
            TransactionError::UserReturnedError(err) => err,
            TransactionError::CouldntCreateTransaction(err) => Error::Sql(err),
        }
    }
}

#[derive(Queryable)]
struct Credentials {
    password: String,
    salt: String,
}

fn authenticate(conn: &Connection, account: String, password: String, addr: String)
                -> Result<(AccountData, HashMap<i16, i8>), Error> {
    use shared::database::schema::{accounts, ip_bans, character_counts};

    let ip_ban: Option<i32> = try!(
        ip_bans::table.filter(ip_bans::ip.eq(&addr))
                      .select_sql::<types::Integer>("1")
                      .first(conn)
                      .optional()
    );

    if ip_ban.is_some() {
        return Err(Error::Reason(identification_failure_reason::WRONG_CREDENTIALS));
    }

    let filter = accounts::table.filter(accounts::account.eq(&account));

    let ban_end: i64 = match try!(filter.select(accounts::ban_end)
                                        .first(conn)
                                        .optional()) {
        Some(ban_end) => ban_end,
        None => return Err(Error::Reason(identification_failure_reason::WRONG_CREDENTIALS)),
    };

    if ban_end < 0 {
        return Err(Error::Reason(identification_failure_reason::BANNED));
    }

    if ban_end > time::get_time().sec {
        return Err(Error::Banned(ban_end));
    }

    let credentials: Credentials = try!(
        filter.select((accounts::password, accounts::salt))
              .first(conn)
    );

    if crypto::md5(&(crypto::md5(&password) + &credentials.salt)) != credentials.password {
        return Err(Error::Reason(identification_failure_reason::WRONG_CREDENTIALS));
    }

    let account: AccountData = try!(
        filter.select((
            accounts::id,
            accounts::account,
            accounts::nickname,
            accounts::secret_question,
            accounts::level,
            accounts::subscription_end,
            accounts::creation_date,
            accounts::already_logged,
            accounts::last_server
        )).first(conn)
    );

    let mut character_counts = HashMap::new();
    let _ = try!(
        character_counts::table.filter(character_counts::account_id.eq(&account.id))
                               .select(character_counts::server_id)
                               .load::<i16>(conn)
    ).map(|serv_id| *character_counts.entry(serv_id).or_insert(0) += 1).count();

    Ok((account, character_counts))
}

impl Session {
    fn identification_success(&mut self, chunk: &ChunkImpl, data: AccountData,
                              character_counts: HashMap<i16, i8>,
                              already_logged: bool, auto_connect: bool) {
        let already_logged = already_logged || data.already_logged != 0;
        log_info!(
            self,
            "connection: ip = {}, already_logged = {}",
            self.base.address,
            already_logged
        );

        self.queue_state = QueueState::None;
        self.account = Some(data);
        self.character_counts = character_counts;
        let data = self.account.as_ref().unwrap();
        let subscriber = data.is_subscriber();

        let mut buf = LoginQueueStatusMessage {
            position: 0,
            total: 0,
        }.as_packet().unwrap();

        IdentificationSuccessMessage {
            has_rights: Flag(data.level > 0),
            was_already_connected: Flag(already_logged),
            login: data.account.clone(),
            nickname: data.nickname.clone(),
            account_id: data.id,
            community_id: 0,
            secret_question: data.secret_question.clone(),
            account_creation: (data.creation_date * 1000) as f64,
            subscription_elapsed_duration: 0.,
            subscription_end_date: match subscriber {
                true => data.subscription_end * 1000,
                false => 0,
            } as f64,
            havenbag_available_room: 0,
        }.as_packet_with_buf(&mut buf).unwrap();

        write!(SERVER, self.base.token, buf);

        if auto_connect && self.select_server(chunk, data.last_server).ok().is_some() {
            return;
        }

        let servers = SERVER.with(|s| s.game_servers.values().filter_map(|server| {
            if server.min_level() > data.level {
                return None;
            }

            let status = chunk.game_status.get(&server.id())
                                          .map(|status| status.0)
                                          .unwrap_or(server_status::OFFLINE);

            Some(self.get_server_informations(server, status))
        }).collect());

        let buf = ServersListMessage {
            servers: servers,
            already_connected_to_server_id: VarShort(data.already_logged),
            can_create_new_character: true,
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);
    }
}

#[register_handlers]
impl Session {
    pub fn handle_identification<'a>(&mut self, _: Ref<'a>, msg: IdentificationMessage)
                                     -> io::Result<()> {
        use std::io::Read;
        use protocol::io::ReadExt;
        use shared::net::Msg;

        if self.account.is_some() || !self.queue_state.is_none() {
            return Ok(());
        }

        let mut key = PKey::new();
        let raw_priv_key = SERVER.with(|s| (*s.priv_key).clone());
        key.load_priv(&raw_priv_key[0..]);

        if msg.credentials.0.len() != 256 {
            return Ok(());
        }

        let mut credentials = io::Cursor::new(
            key.decrypt_with_padding(&msg.credentials.0[0..], EncryptionPadding::PKCS1v15)
        );

        let mut salt = vec![0; self.salt.len()];
        try!(credentials.read_exact(&mut salt));

        self.aes_key = vec![0; 32];
        try!(credentials.read_exact(&mut self.aes_key));

        if msg.use_certificate.0 {
            let _ = try!(credentials.read_i32());
            let mut certificate = vec![0; 32];
            try!(credentials.read_exact(&mut certificate));
        }

        let len = try!(credentials.read_i8());
        let mut username = vec![0; len as usize];
        try!(credentials.read_exact(&mut username));
        let username = match String::from_utf8(username) {
            Ok(username) => username,
            Err(_) => return Ok(()),
        };

        let mut password = Vec::new();
        try!(credentials.read_to_end(&mut password));
        let password = match String::from_utf8(password) {
            Ok(password) => password,
            Err(_) => return Ok(()),
        };

        let auto_connect = msg.autoconnect.0;

        let token = self.base.token;
        let addr = self.base.address.clone();
        let io_loop = SERVER.with(|s| s.io_loop.clone());
        let server = SERVER.with(|s| s.server.clone());

        self.queue_state = QueueState::Some(
            QUEUE_SIZE.fetch_add(1, Ordering::Relaxed) + 1,
            QUEUE_COUNTER.load(Ordering::Relaxed)
        );

        SERVER.with(|s| database::execute(&s.db, move |conn| {
            let res = conn.transaction(|| {
                authenticate(conn, username, password, addr)
            }).map_err(From::from);

            match res {
                Err(err) => {
                    let buf = match err {
                        Error::Banned(ban_end) =>
                            IdentificationFailedBannedMessage {
                                base: IdentificationFailedMessage {
                                    reason: identification_failure_reason::BANNED,
                                },
                                ban_end_date: (ban_end * 1000) as f64,
                            }.as_packet().unwrap(),

                        Error::Reason(reason) =>
                            IdentificationFailedMessage {
                                reason: reason,
                            }.as_packet().unwrap(),

                        Error::Sql(err) => {
                            error!("authenticate sql error: {}", err);
                            IdentificationFailedMessage {
                                reason: identification_failure_reason::UNKNOWN_AUTH_ERROR,
                            }.as_packet().unwrap()
                        }
                    };

                    let _ = io_loop.send(Msg::WriteAndClose(token, buf));
                }

                Ok((data, counts)) => {
                    let id = data.id;
                    server::identification_success(
                        &server,
                        token,
                        id,
                        data.already_logged,
                        move |session, chunk, already| {
                            session.identification_success(
                                chunk,
                                data,
                                counts,
                                already,
                                auto_connect
                            )
                        }
                    );
                }
            }

            let _ = QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed);
            let _ = QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
        }));

        Ok(())
    }
}
