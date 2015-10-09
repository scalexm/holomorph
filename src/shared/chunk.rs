use std::thread::{self, JoinHandle};
use std::sync::mpsc;
use std::boxed::FnBox;
use std::collections::LinkedList;

pub enum Msg<C> {
    Shutdown,
    ChunkCallback(Thunk<C>),
}

pub type Thunk<C> = Box<FnBox(&mut C) + Send + 'static>;
pub type Sender<C> = mpsc::Sender<Msg<C>>;

// helper function to convert an FnOnce into an FnBox and send it to a chunk
pub fn send<F, C>(sender: &Sender<C>, job: F) where F : FnOnce(&mut C) + Send + 'static {
    let boxed_job: Thunk<C> = Box::new(move |chunk: &mut C| job(chunk));
    let _ = sender.send(Msg::ChunkCallback(boxed_job));
}

pub fn run<C: Send + 'static>(mut chunk: C, joins: &mut LinkedList<JoinHandle<()>>) -> Sender<C> {
    let (tx, rx) = mpsc::channel::<Msg<C>>();
    joins.push_back(thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(msg) => {
                        match msg {
                            Msg::Shutdown => return,
                            Msg::ChunkCallback(thunk) => thunk.call_box((&mut chunk,)),
                        }
                    }
                    Err(..) => return,
                }
            }
    }));

    tx
}
