pub mod session;

use std::thread;
use std::sync::mpsc;
use std::boxed::FnBox;

// marker trait
pub trait Chunk {
    fn process_cb(&mut self, thunk: Thunk<Self>) {
        thunk.call_box((self, ))
    }
}

pub enum Msg<C: Chunk> {
    Shutdown,
    ChunkCallback(Thunk<C>),
}

pub type Thunk<C> = Box<FnBox(&mut C) + Send + 'static>;
pub type Sender<C> = mpsc::Sender<Msg<C>>;

// helper function to convert an FnOnce into an FnBox and send it to a chunk
pub fn execute<F, C: Chunk>(sender: &Sender<C>, job: F)
    where F : FnOnce(&mut C) + Send + 'static {

    let boxed_job: Thunk<C> = Box::new(move |chunk: &mut C| job(chunk));
    let _ = sender.send(Msg::ChunkCallback(boxed_job));
}

pub fn run_chunk<C: Chunk + Send + 'static>(mut chunk: C)
    -> Sender<C> {
    let (tx, rx) = mpsc::channel::<Msg<C>>();

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(msg) => {
                    match msg {
                        Msg::Shutdown => return (),
                        Msg::ChunkCallback(thunk) => chunk.process_cb(thunk),
                    }
                }
                Err(..) => return (),
            }
        }
    });

    tx
}
