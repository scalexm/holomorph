pub mod chunk;

use net::Token;
use std::io::{self, Cursor};
use self::chunk::Ref;
use std::collections::LinkedList;
use std::cell::RefCell;
use postgres::{self, Connection};
use time;

struct SessionLog {
    date: i64,
    type_: String,
    content: String,
}

#[macro_export]
macro_rules! log_custom {
    ($session: expr, $type_: expr, $($arg: tt)*) => {{
        $session.base.push_log($type_.to_string(), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! log_err {
    ($session: expr, $($arg : tt)*) => { log_custom!($session, "error", $($arg)*); };
}

#[macro_export]
macro_rules! log_info {
    ($session: expr, $($arg : tt)*) => { log_custom!($session, "info", $($arg)*); };
}

// base Session class
pub struct SessionBase {
    pub token: Token,
    pub address: String,
    logs: RefCell<LinkedList<SessionLog>>,
}

pub trait Session<U>: Sized {
    fn new(SessionBase) -> Self;

    fn get_handler<'a>(u16) -> fn(&mut Self, Ref<'a, Self, U>, Cursor<Vec<u8>>)
                                  -> io::Result<()>;

    fn unhandled<'a>(&mut self, _: Ref<'a, Self, U>, _: Cursor<Vec<u8>>) -> io::Result<()> {
        Ok(())
    }

    fn close<'a>(self, Ref<'a, Self, U>);
}

impl SessionBase {
    fn new(token: Token, address: String) -> Self {
        debug!("{:?} connected", token);

        SessionBase {
            token: token,
            address: address,
            logs: RefCell::new(LinkedList::new()),
        }
    }

    pub fn save_logs(&self, conn: &mut Connection, account_id: i32) -> postgres::Result<()> {
        let trans = try!(conn.transaction());
        for log in &*self.logs.borrow() {
            let stmt = try!(trans.prepare_cached("INSERT INTO logs(account_id, date, type,
                content) VALUES($1, $2, $3, $4)"));
            let _ = try!(stmt.execute(&[&account_id, &log.date, &log.type_, &log.content]));
        }
        trans.commit()
    }

    pub fn push_log(&self, type_: String, content: String) {
        self.logs.borrow_mut().push_back(SessionLog {
            date: time::precise_time_ns() as i64,
            type_: type_,
            content: content,
        });
    }
}

impl Drop for SessionBase {
    fn drop(&mut self) {
        debug!("{:?} logout", self.token);
    }
}
