mod handlers;

use shared::net::Token;
use shared::pool;
use session;
use std::io::{self, Cursor};
use rand::{self, Rng};

pub struct Session {
    token: Token,
    server_id: Option<i16>,
    salt: String,
    ip: String,
    port: u16,
}

pub type Chunk = session::Chunk<Session>;
pub type Sender = pool::Sender<Chunk>;

impl Drop for Session {
    fn drop(&mut self) {
        debug!("game server {:?} logout", self.token);
    }
}

impl pool::session::Session for Session {
    type C = Chunk;

    fn new(token: Token, chunk: &Chunk) -> Session {
        debug!("game server {:?} connected", token);

        let s = Session {
            token: token,
            server_id: None,
            salt: rand::thread_rng().gen_ascii_chars().take(32).collect(),
            ip: String::new(),
            port: 0,
        };

        s.start(chunk);
        s
    }

    fn get_handler(id: u16)
        -> (fn(&mut Session, &Chunk, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            2 => Session::handle_identification,
            3 => Session::handle_state,
            _ => Session::unhandled,
        }
    }
}
