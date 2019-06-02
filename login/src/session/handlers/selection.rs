use crate::session::{Session, State, TcpStreamExt, AES_IV_LEN};
use log::{debug, error};
use protocol::messages::connection::ServerSelectionMessage;

const TICKET_LEN: usize = 32;

impl Session {
    pub async fn select_server(
        &mut self,
        server_id: i16,
    ) -> std::io::Result<Result<(), ServerSelectionError>> {
        use protocol::constants::server_connection_error;
        use protocol::constants::server_status;
        use protocol::messages::connection::SelectedServerDataMessage;
        use rand::Rng;

        let (account, aes_key) = match &self.state {
            State::Logged { account, aes_key } => (account, aes_key),
            _ => return Ok(Ok(())),
        };

        let gs = match self.server.game_servers.get(&server_id) {
            Some(gs) => gs,
            None => {
                return Ok(Err(ServerSelectionError::KnownReason(
                    server_connection_error::NO_REASON,
                )));
            }
        };

        if gs.status() != server_status::ONLINE {
            return Ok(Err(ServerSelectionError::KnownReason(
                server_connection_error::DUE_TO_STATUS,
            )));
        }

        let mut ticket: String = {
            let mut rng = rand::thread_rng();
            std::iter::repeat(())
                .map(|()| rng.sample(rand::distributions::Alphanumeric))
                .take(TICKET_LEN)
                .collect()
        };

        let encrypted = match openssl::symm::encrypt(
            openssl::symm::Cipher::aes_256_cbc(),
            aes_key,
            Some(&aes_key[..AES_IV_LEN]),
            ticket.as_bytes(),
        ) {
            Ok(encrypted) => encrypted,
            Err(err) => return Ok(Err(ServerSelectionError::EncryptionError(err))),
        };

        let account_id = account.id;

        debug!(
            "server selection with ticket = {}, moving to background thread",
            ticket
        );
        let server = &self.server;

        // FIXME: once diesel starts integrating with async runtimes, we won't
        // need this `blocking` call.
        let fut = super::compat_poll_fn(|| {
            tokio_threadpool::blocking(|| {
                use database::accounts::dsl;
                use diesel::prelude::*;

                let conn = server
                    .database_pool
                    .get()
                    .map_err(ServerSelectionError::DatabasePoolError)?;

                diesel::update(dsl::accounts.find(account_id))
                    .set((
                        dsl::ticket.eq(std::mem::replace(&mut ticket, String::new())),
                        dsl::last_server.eq(Some(server_id)),
                    ))
                    .execute(&conn)
                    .map_err(ServerSelectionError::SqlError)
            })
        })
        .await
        .unwrap();

        if let Err(err) = fut {
            return Ok(Err(err));
        }

        debug!("server selected: {}", gs.id());

        self.stream
            .send_msg(SelectedServerDataMessage {
                server_id: gs.id() as _,
                address: gs.host(),
                ports: std::borrow::Cow::Borrowed(&[gs.port() as u32]),
                can_create_new_character: true,

                // Just convert from an `&[u8]` to an `&[i8]`.
                ticket: unsafe {
                    std::slice::from_raw_parts(encrypted.as_ptr() as *const i8, encrypted.len())
                },
            })
            .await?;
        self.stream.get_ref().shutdown(std::net::Shutdown::Both)?;

        Ok(Ok(()))
    }

    pub async fn handle_server_selection<'a>(
        &'a mut self,
        msg: ServerSelectionMessage<'a>,
    ) -> std::io::Result<()> {
        use protocol::constants::server_connection_error;
        use protocol::messages::connection::SelectedServerRefusedMessage;

        if let Err(err) = self.select_server(msg.server_id as _).await? {
            let reason = match err {
                ServerSelectionError::KnownReason(reason) => reason,
                err => {
                    error!("unexpected error: {}", err);
                    server_connection_error::NO_REASON
                }
            };

            self.stream
                .send_msg(SelectedServerRefusedMessage {
                    server_id: msg.server_id,
                    error: reason,
                    server_status: self
                        .server
                        .game_servers
                        .get(&(msg.server_id as _))
                        .map(|gs| gs.status())
                        .unwrap_or(0),
                    _phantom: std::marker::PhantomData,
                })
                .await?;
        }

        Ok(())
    }
}

pub enum ServerSelectionError {
    EncryptionError(openssl::error::ErrorStack),
    SqlError(diesel::result::Error),
    DatabasePoolError(diesel::r2d2::PoolError),
    KnownReason(u8),
}

impl std::fmt::Display for ServerSelectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerSelectionError::EncryptionError(err) => write!(f, "{}", err),
            ServerSelectionError::DatabasePoolError(err) => write!(f, "{}", err),
            ServerSelectionError::SqlError(err) => write!(f, "{}", err),
            ServerSelectionError::KnownReason(err) => write!(f, "{}", err),
        }
    }
}
