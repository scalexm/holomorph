use postgres::{Connection, SslMode};
use std::sync::mpsc::{channel, Sender};
use std::thread;

trait FnBox {
    fn call_box(self: Box<Self>, &mut Connection);
}

impl<F: FnOnce(&mut Connection)> FnBox for F {
    fn call_box(self: Box<F>, conn: &mut Connection) {
        (*self)(conn)
    }
}

pub fn connect(uri: &str) -> Connection {
    match Connection::connect(uri, &SslMode::None) {
        Ok(conn) => conn,
        Err(err) => panic!("database connection failed: {}", err),
    }
}

pub type Thunk = Box<FnBox + Send + 'static>;

pub fn async_connect(uri: &str) -> Sender<Thunk> {
    let (tx, rx) = channel::<Thunk>();
    let mut conn = connect(uri);

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(job) => job.call_box(&mut conn),
                Err(..) => return (),
            }
        }
    });

    tx
}
