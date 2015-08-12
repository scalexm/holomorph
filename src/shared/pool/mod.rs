pub mod session;

use std::thread;
use std::sync::mpsc;
use net::Token;
use std::io::Cursor;
use std::boxed::FnBox;

pub trait Chunk {
    fn process_net_msg(&mut self, msg: NetMsg);

    fn process_cb(&mut self, thunk: Thunk<Self>) {
        thunk.call_box((self, ))
    }
}

pub enum NetMsg {
    SessionConnect(Token),
    SessionPacket(Token, u16, Cursor<Vec<u8>>),
    SessionDisconnect(Token),
}

pub enum Msg<C: Chunk> {
    Shutdown,
    NetMsg(NetMsg),
    ChunkCallback(Thunk<C>),
}

impl<C: Chunk> Into<Msg<C>> for NetMsg {
    fn into(self) -> Msg<C> {
        Msg::NetMsg(self)
    }
}

pub type Thunk<C: Chunk> = Box<FnBox(&mut C) + Send + 'static>;
pub type Sender<C: Chunk> = mpsc::Sender<Msg<C>>;

pub fn execute<F, C: Chunk>(sender: &Sender<C>, job: F)
    where F : FnOnce(&mut C) + Send + 'static {

    let boxed_job: Thunk<C> =
        Box::new(move |chunk: &mut C| job(chunk));
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
                        Msg::NetMsg(msg) => chunk.process_net_msg(msg),
                        Msg::ChunkCallback(thunk) => chunk.process_cb(thunk),
                    }
                }
                Err(..) => return (),
            }
        }
    });

    tx
}
