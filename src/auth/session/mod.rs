pub mod auth;
pub mod game;

use shared::pool;
use shared::net::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use server::data::AuthServerData;

struct ServerStatus(i8, String, i16);

struct Chunk<S> {
    sessions: HashMap<Token, RefCell<S>>,
    server: AuthServerData,
    game_status: HashMap<i16, ServerStatus>,
}

impl<S> Chunk<S> {
    pub fn new(server: AuthServerData) -> Chunk<S> {
        Chunk {
            sessions: HashMap::new(),
            server: server,
            game_status: HashMap::new(),
        }
    }
}

impl<S> pool::Chunk for Chunk<S> { }

impl<S: pool::session::Session<C = Chunk<S>>> pool::session::Chunk for Chunk<S> {
    type S = S;

    fn sessions(&self) -> &HashMap<Token, RefCell<S>> {
        &self.sessions
    }

    fn mut_sessions(&mut self) -> &mut HashMap<Token, RefCell<S>> {
        &mut self.sessions
    }
}
