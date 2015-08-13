use shared::pool;
use shared::net::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use server::data::AuthServerData;

pub struct Chunk<S> {
    sessions: HashMap<Token, RefCell<S>>,
    pub server: AuthServerData,
}

impl<S> Chunk<S> {
    pub fn new(server: AuthServerData) -> Chunk<S> {
        Chunk {
            sessions: HashMap::new(),
            server: server,
        }
    }

    pub fn session_callback<F>(&mut self, tok: Token, job: F)
        where F: FnOnce(&mut S, &Chunk<S>) {

        if let Some(session) = self.sessions.get(&tok) {
            job(&mut session.borrow_mut(), self)
        }
    }
}

impl<S: pool::session::Session<C = Chunk<S>>> pool::Chunk for Chunk<S> {
    fn process_net_msg(&mut self, msg: pool::NetMsg) {
        pool::session::Chunk::process_net_msg(self, msg)
    }
}

impl<S: pool::session::Session<C = Chunk<S>>> pool::session::Chunk for Chunk<S> {
    type S = S;

    fn sessions(&self) -> &HashMap<Token, RefCell<S>> {
        &self.sessions
    }

    fn mut_sessions(&mut self) -> &mut HashMap<Token, RefCell<S>> {
        &mut self.sessions
    }
}
