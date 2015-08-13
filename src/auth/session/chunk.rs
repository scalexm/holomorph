use shared::pool;
use session::{self, Session};
use shared::net::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use server::data::AuthServerData;
use std::boxed::FnBox;

pub struct Chunk {
    sessions: HashMap<Token, RefCell<Session>>,
    pub server: AuthServerData,
}

impl Chunk {
    pub fn new(server: AuthServerData) -> Chunk {
        Chunk {
            sessions: HashMap::new(),
            server: server,
        }
    }

    pub fn session_callback<F>(&mut self, tok: Token, job: F)
        where F: FnOnce(&mut Session, &Chunk) {

        if let Some(session) = self.sessions.get(&tok) {
            job(&mut session.borrow_mut(), self)
        }
    }
}

pub type Sender = pool::Sender<Chunk>;

impl pool::Chunk for Chunk {
    fn process_net_msg(&mut self, msg: pool::NetMsg) {
        pool::session::Chunk::process_net_msg(self, msg)
    }
}

impl pool::session::Chunk for Chunk {
    type S = Session;

    fn sessions(&self) -> &HashMap<Token, RefCell<Session>> {
        &self.sessions
    }

    fn mut_sessions(&mut self) -> &mut HashMap<Token, RefCell<Session>> {
        &mut self.sessions
    }
}
