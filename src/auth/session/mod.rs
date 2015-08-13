mod handlers;

use shared::net::Token;
use std::io::{self, Cursor};
use shared::pool;
use chunk;
use std::collections::HashMap;

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

pub type Chunk = chunk::Chunk<Session>;
pub type Sender = pool::Sender<Chunk>;

impl Drop for Session {
    fn drop(&mut self) {
        debug!("{:?} logout", self.token);
    }
}

impl pool::session::Session for Session {
    type C = Chunk;

    fn new(token: Token, chunk: &Chunk) -> Session {
        debug!("{:?} connected", token);

        let mut s = Session {
            token: token,
            account: None,
        };

        s.start(&chunk);
        s
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
