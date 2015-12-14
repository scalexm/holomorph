pub mod chunk;

use net::Token;
use std::io;
use self::chunk::Ref;
use std::collections::LinkedList;
use std::cell::RefCell;
use diesel::*;
use time;
use database::schema::logs;

struct SessionLog {
    date: i64,
    log_type: String,
    content: String,
}

#[insertable_into(logs)]
struct NewLog<'a>(
    #[column_name="account_id"]
    i32,
    #[column_name="date"]
    i64,
    #[column_name="log_type"]
    &'a str,
    #[column_name="content"]
    &'a str
);

impl<'a> NewLog<'a> {
    fn new(account_id: i32, log: &'a SessionLog) -> Self {
        NewLog(account_id, log.date, &log.log_type, &log.content)
    }
}

#[macro_export]
macro_rules! log_custom {
    ($session: expr, $ty: expr, $($arg: tt)*) => {{
        $session.base.push_log($ty.to_string(), format!($($arg)*));
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

    fn handle<'a>(&mut self, Ref<'a, Self, U>, i16, io::Cursor<Vec<u8>>) -> io::Result<()>;

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

    pub fn save_logs(&self, conn: &Connection, account_id: i32) -> QueryResult<()> {
        use diesel::query_builder::insert;
        try!(conn.transaction(|| {
            let logs = self.logs.borrow();
            let rows: Vec<_> = logs.iter()
                                   .map(|log| NewLog::new(account_id, log))
                                   .collect();
            insert(&rows).into(logs::table).execute(conn)
        }));
        Ok(())
    }

    pub fn push_log(&self, log_type: String, content: String) {
        self.logs.borrow_mut().push_back(SessionLog {
            date: time::get_time().sec,
            log_type: log_type,
            content: content,
        });
    }
}

impl Drop for SessionBase {
    fn drop(&mut self) {
        debug!("{:?} logout", self.token);
    }
}
