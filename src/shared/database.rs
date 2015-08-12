use postgres::{Connection, SslMode};
use std::sync::mpsc;
use std::thread;
use std::boxed::FnBox;

pub fn connect(uri: &str) -> Connection {
    match Connection::connect(uri, &SslMode::None) {
        Ok(conn) => conn,
        Err(err) => panic!("database connection failed: {}", err),
    }
}

type Thunk = Box<FnBox(&mut Connection) + Send + 'static>;
pub type Sender = mpsc::Sender<Thunk>;

pub fn execute<F>(sender: &Sender, job: F)
    where F : FnOnce(&mut Connection) + Send + 'static {

    let boxed_job = Box::new(move |conn: &mut Connection| job(conn));
    let _ = sender.send(boxed_job);
}

pub fn async_connect(uri: &str) -> Sender {
    let (tx, rx) = mpsc::channel::<Thunk>();
    let mut conn = connect(uri);

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(job) => job.call_box((&mut conn,)),
                Err(..) => return (),
            }
        }
    });

    tx
}
