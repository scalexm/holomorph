mod handlers;

use crate::server::Server;
use log::{debug, error};
use protocol::frame::Framed;
use protocol::Decode;
use runtime::net::TcpStream;
use std::sync::Arc;

const SALT_LEN: usize = 32;
const AES_KEY_LEN: usize = 32;
const AES_IV_LEN: usize = 16;

#[derive(PartialEq, Eq, Debug, Queryable)]
struct Account {
    id: i32,
    login: String,
    nickname: String,
    last_server: Option<i16>,
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Init,
    Logged {
        account: Account,
        aes_key: [u8; AES_KEY_LEN],
    },
}

pub struct Session {
    stream: Framed<TcpStream>,
    state: State,
    server: Arc<Server>,
}

impl Session {
    pub fn new(stream: TcpStream, server: Arc<Server>) -> Self {
        Self {
            stream: Framed::new(stream),
            state: State::Init,
            server,
        }
    }

    pub async fn run(mut self) -> std::io::Result<()> {
        use futures::StreamExt;
        use protocol::messages::connection::HelloConnectMessage;
        use protocol::messages::connection::IdentificationMessage;
        use protocol::messages::connection::ServerSelectionMessage;
        use protocol::messages::handshake::ProtocolRequired;
        use rand::Rng;

        debug!(
            "new connection from {:?}",
            self.stream.get_ref().peer_addr()
        );

        self.stream.write(ProtocolRequired {
            required_version: 1924,
            current_version: 1924,
            _phantom: std::marker::PhantomData,
        })?;

        let salt: String = {
            let mut rng = rand::thread_rng();
            std::iter::repeat(())
                .map(|()| rng.sample(rand::distributions::Alphanumeric))
                .take(SALT_LEN)
                .collect()
        };

        self.stream.write(HelloConnectMessage {
            salt: &salt,
            key: unsafe {
                std::slice::from_raw_parts(
                    self.server.public_key.as_ptr() as *const i8,
                    self.server.public_key.len(),
                )
            },
        })?;

        self.stream.flush().await?;

        while let Some(frame) = self.stream.next().await {
            let frame = frame?;

            debug!("received message with id {}", frame.id());

            match frame.id() {
                <IdentificationMessage<'_> as Decode<'_>>::ID => {
                    match IdentificationMessage::decode(&mut frame.payload()) {
                        Ok(msg) => self.handle_identification(msg).await?,
                        Err(err) => error!("decode error: {}", err),
                    }
                }

                <ServerSelectionMessage<'_> as Decode<'_>>::ID => {
                    match ServerSelectionMessage::decode(&mut frame.payload()) {
                        Ok(msg) => self.handle_server_selection(msg).await?,
                        Err(err) => error!("decode error: {}", err),
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}
