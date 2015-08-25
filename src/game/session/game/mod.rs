mod handlers;

use std::collections::HashMap;
use shared::pool;
use shared::net::Token;
use std::io::{self, Cursor};
use std::cell::RefCell;
use server::data::GameServerData;

pub struct Chunk {
    sessions: HashMap<Token, RefCell<Session>>,
    server: GameServerData,
}

impl Chunk {
    pub fn new(server: GameServerData) -> Chunk {
        Chunk {
            sessions: HashMap::new(),
            server: server,
        }
    }
}

impl pool::Chunk for Chunk { }

impl pool::session::Chunk for Chunk {
    type S = Session;

    fn sessions(&self) -> &HashMap<Token, RefCell<Session>> {
        &self.sessions
    }

    fn mut_sessions(&mut self) -> &mut HashMap<Token, RefCell<Session>> {
        &mut self.sessions
    }
}

pub type Sender = pool::Sender<Chunk>;

pub struct Session {
    token: Token,
    address: String,
}

impl Drop for Session {
    fn drop(&mut self) {
        debug!("{:?} logout", self.token);
    }
}

impl pool::session::Session for Session {
    type C = Chunk;

    fn new(token: Token, chunk: &Chunk, address: String) -> Session {
        debug!("{:?} connected", token);

        let s = Session {
            token: token,
            address: address,
        };

        s.start(chunk);
        s
    }

    fn get_handler(id: u16)
        -> (fn(&mut Session, &Chunk, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            110 => Session::handle_authentication_ticket,
            _ => Session::unhandled,
        }
    }
}
