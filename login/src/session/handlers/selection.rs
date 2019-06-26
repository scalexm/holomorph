use crate::session::{Session, State, AES_IV_LEN};
use log::{debug, error};
use protocol::messages::connection::ServerSelectionMessage;

impl Session {
    pub async fn select_server(&mut self, server_id: i16) -> std::io::Result<u8> {
        use protocol::constants::server_connection_error;
        use protocol::constants::server_status;
        use protocol::messages::connection::SelectedServerDataMessage;

        let (aes_key, ticket) = match &self.state {
            State::Logged { aes_key, ticket } => (aes_key, ticket),
            _ => return Ok(0),
        };

        let gs = match self.server.game_servers.get(&server_id) {
            Some(gs) => gs,
            None => return Ok(server_connection_error::NO_REASON),
        };

        if gs.status() != server_status::ONLINE {
            return Ok(server_connection_error::DUE_TO_STATUS);
        }

        let encrypted = match openssl::symm::encrypt(
            openssl::symm::Cipher::aes_256_cbc(),
            aes_key,
            Some(&aes_key[..AES_IV_LEN]),
            ticket.as_bytes(),
        ) {
            Ok(encrypted) => encrypted,
            Err(err) => {
                error!("encryption error: {}", err);
                return Ok(server_connection_error::NO_REASON);
            }
        };

        debug!("server selected: {}, ticket = {}", gs.id(), ticket);

        let ports = &[gs.port() as u32];
        self.stream.write(SelectedServerDataMessage {
            server_id: gs.id() as _,
            address: gs.host(),
            ports: std::borrow::Cow::Borrowed(ports),
            can_create_new_character: true,

            // Just convert from an `&[u8]` to an `&[i8]`.
            ticket: unsafe {
                std::slice::from_raw_parts(encrypted.as_ptr() as *const i8, encrypted.len())
            },
        })?;
        self.stream.flush().await?;
        self.stream.get_ref().shutdown(std::net::Shutdown::Both)?;

        Ok(0)
    }

    pub async fn handle_server_selection<'a>(
        &'a mut self,
        msg: ServerSelectionMessage<'a>,
    ) -> std::io::Result<()> {
        use protocol::messages::connection::SelectedServerRefusedMessage;

        let reason = self.select_server(msg.server_id as _).await?;
        if reason != 0 {
            self.stream
                .send(SelectedServerRefusedMessage {
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
