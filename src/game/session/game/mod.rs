mod handlers;

use std::collections::HashMap;
use shared::pool;
use shared::net::Token;
use std::io::{self, Cursor};
use std::cell::RefCell;
use server::data::GameServerData;
use postgres::{self, Connection};
use time;
use shared::database;

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

    pub fn update_queue(&self) {
        for session in &self.sessions {
            session.1.borrow().update_queue(self);
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

enum QueueState {
    None,
    SomeTicket(isize, isize),
    SomeGame(isize, isize),
}

impl QueueState {
    fn is_none(&self) -> bool {
        match *self {
            QueueState::None => true,
            _ => false,
        }
    }
}

struct AccountData {
    id: i32,
    nickname: String,
    secret_answer: String,
    level: i8,
    subscription_end: i64,
    last_connection: i64,
    last_ip: String,
}

impl AccountData {
    fn is_subscriber(&self) -> bool {
        self.subscription_end > time::get_time().sec
    }
}

pub struct Session {
    token: Token,
    address: String,
    queue_state: QueueState,
    account: Option<AccountData>,
}

impl Session {
    fn save_auth(&mut self, conn: &mut Connection) -> postgres::Result<()> {

        if self.account.is_none () {
            return Ok(());
        }

        let account = self.account.as_ref().unwrap();
        let stmt = try!(conn.prepare_cached("UPDATE accounts SET logged = 0
            WHERE id = $1"));
        try!(stmt.execute(&[&account.id]));

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

    fn new(token: Token, chunk: &Chunk, address: String) -> Session {
        debug!("{:?} connected", token);

        let s = Session {
            token: token,
            address: address,
            queue_state: QueueState::None,
            account: None,
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

    fn close(mut self, chunk: &Chunk) {
        database::execute(&chunk.server.auth_db, move |conn| {
            if let Err(err) = self.save_auth(conn) {
                error!("error while saving session to auth db: {:?}", err);
            }
        });
    }
}
