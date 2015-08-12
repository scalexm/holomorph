mod handlers;
pub mod chunk;

use shared::net::{Token, Msg};
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::connection::*;
use shared::protocol::handshake::*;
use shared::pool;
use session::chunk::Chunk;
use std::boxed::FnBox;
use std::collections::HashMap;

pub type Thunk = Box<FnBox(&mut Session, &Chunk) + Send + 'static>;

struct AccountData {
    id: i32,
    account: String,
    nickname: String,
    secret_question: String,
    level: i8,
    subscription_end: i64,
    subscription_elapsed: i64,
    creation: i64,
    character_counts: HashMap<i16, i8>,
    already_logged: i16,
}

pub struct Session {
    token: Token,
    account: Option<AccountData>,
}

impl Session {
    fn start(&mut self, chunk: &Chunk) -> io::Result<()> {

        let mut buf = Vec::new();
        try!(ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet_with_buf(&mut buf));

        try!(HelloConnectMessage {
            salt: "salut".to_string(),
            key: VarIntVec(chunk.server.key[0..].to_vec()),
        }.as_packet_with_buf(&mut buf));

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        debug!("{:?} logout", self.token);
    }
}

impl pool::session::Session for Session {
    type C = Chunk;

    fn new(token: Token, chunk: &Chunk) -> Option<Session> {
        debug!("{:?} connected", token);

        let mut s = Session {
            token: token,
            account: None,
        };

        if let Err(err) = s.start(&chunk) {
            error!("error while starting {:?}: {}", token, err);
            error!("{:?} will disconnect", token);
            return None;
        }

        Some(s)
    }

    fn get_handler(id: u16)
        -> (fn(&mut Session, &Chunk, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            4 => Session::handle_identification,
            888 => Session::handle_clear_identification,
            _ => Session::unhandled,
        }
    }
}
