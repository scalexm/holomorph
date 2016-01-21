pub mod schema;

use diesel::*;
use std::sync::{Arc, Mutex, mpsc};
use std::boxed::FnBox;
use std::thread::{self, JoinHandle};
use std::collections::LinkedList;

pub type Thunk = Box<FnBox(&Connection) + Send + 'static>;
pub type Sender = mpsc::Sender<Thunk>;

// starts a thread pool
pub fn spawn_threads(threads: usize, uri: &str, joins: &mut LinkedList<JoinHandle<()>>)
                     -> Sender {
    assert!(threads >= 1);

    let (tx, rx) = mpsc::channel::<Thunk>();
    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..threads {
        let rx = rx.clone();
        let uri = uri.to_string();

        joins.push_back(thread::spawn(move || {
            let conn = Connection::establish(&uri).unwrap();
            loop {
                // we acquire the lock only for receiving, not for executing a job
                let msg = {
                    let lock = rx.lock().unwrap();
                    lock.recv()
                };

                match msg {
                    Ok(job) => job.call_box((&conn,)),
                    Err(..) => return,
                }
            }
        }));
    }

    tx
}

// helper function to convert an FnOnce into an FnBox and send it to the pool
pub fn execute<F>(sender: &Sender, job: F) where F : FnOnce(&Connection) + Send + 'static {
    let boxed_job: Thunk = Box::new(move |conn: &Connection| job(conn));
    let _ = sender.send(boxed_job);
}
