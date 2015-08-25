mod handlers;

use shared::net::Token;
use std::io::{self, Cursor};
use shared::pool;
use session::{self, ServerStatus};
use std::collections::HashMap;
use time;

struct AccountData {
    id: i32,
    account: String,
    nickname: String,
    secret_question: String,
    level: i8,
    subscription_end: i64,
    subscription_elapsed: i64,
    creation_date: i64,
    character_counts: HashMap<i16, i8>,
    already_logged: i16,
    last_server: i16,
}

impl AccountData {
    fn is_subscriber(&self) -> bool {
        self.subscription_end > time::get_time().sec
    }
}

pub struct Session {
    token: Token,
    account: Option<AccountData>,
    queue_size: isize,
    queue_counter: isize,
    custom_identification: bool,
    aes_key: Vec<u8>,
    address: String,
}

pub type Chunk = session::Chunk<Session>;
pub type Sender = pool::Sender<Chunk>;

impl Chunk {
    pub fn update_queue(&self) {
        for session in &self.sessions {
            session.1.borrow().update_queue(self);
        }
    }

    pub fn update_game_server(&mut self, server_id: i16, status: i8, ip: String,
        port: i16) {

        let _ = self.game_status.insert(server_id, ServerStatus(status, ip, port));
        for session in &self.sessions {
            session.1.borrow().update_server_status(self, server_id, status);
        }
    }
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
            account: None,
            queue_size: -1,
            queue_counter: -1,
            custom_identification: false,
            aes_key: Vec::new(),
            address: address,
        };

        s.start(chunk);
        s
    }

    fn get_handler(id: u16)
        -> (fn(&mut Session, &Chunk, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            4 => Session::handle_identification,
            40 => Session::handle_server_selection,
            _ => Session::unhandled,
        }
    }
}
