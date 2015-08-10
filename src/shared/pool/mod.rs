pub mod session;

use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use net::Token;
use std::io::Cursor;
use std::collections::HashMap;
use std::any::Any;

pub trait Chunk {
    fn process_msg(&mut self, msg: Msg);
}

pub trait FnBox {
    fn call_box(self: Box<Self>, &mut Any, &Chunk);
}

impl<F: FnOnce(&mut Any, &Chunk)> FnBox for F {
    fn call_box(self: Box<F>, session: &mut Any, chunk: &Chunk) {
        (*self)(session, chunk)
    }
}

pub type Thunk = Box<FnBox + Send + 'static>;

pub enum Msg {
    Shutdown,
    SessionCreate(Token),
    SessionPacket(Token, u16, Cursor<Vec<u8>>),
    SessionRemove(Token),
    SessionCallback(Token, Thunk),
    ChunkCallback(usize, Thunk),
}

pub struct SessionPool {
    core_tx: Sender<Msg>,
    core_rx: Receiver<Msg>,
    chunks: Vec<Sender<Msg>>,
    session_map: HashMap<Token, usize>,
}

impl SessionPool {
    pub fn run_chunk<C: Chunk + Send + 'static>(&mut self, mut chunk: C) {
        let (tx, rx) = channel::<Msg>();
        self.chunks.push(tx);

        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(msg) => chunk.process_msg(msg),
                    Err(..) => return (),
                }
            }
        });
    }

    pub fn new() -> SessionPool {
        let (tx, rx) = channel::<Msg>();

        SessionPool {
            core_tx: tx,
            core_rx: rx,
            chunks: Vec::new(),
            session_map: HashMap::new(),
        }
    }

    pub fn channel(&self) -> Sender<Msg> {
        self.core_tx.clone()
    }

    fn process_msg(&mut self, msg: Msg, next_insert: &mut usize) {
        match msg {
            Msg::SessionCreate(tok) => {
                if self.session_map.contains_key(&tok) {
                    return ();
                }

                let chunk = *next_insert % self.chunks.len();
                let _ = self.session_map.insert(tok, chunk);
                let _  = self.chunks[chunk].send(msg);

                *next_insert += 1;
            }

            Msg::SessionPacket(tok, _, _) => {
                if let Some(chunk) = self.session_map.get(&tok) {
                    let _ = self.chunks[*chunk].send(msg);
                }
            }

            Msg::SessionRemove(tok) => {
                if let Some(chunk) = self.session_map.get(&tok) {
                    let _ = self.chunks[*chunk].send(msg);
                }
                let _ = self.session_map.remove(&tok);
            }

            Msg::SessionCallback(tok, _) => {
                if let Some(chunk) = self.session_map.get(&tok) {
                    let _ = self.chunks[*chunk].send(msg);
                }
            }

            Msg::ChunkCallback(id, _) => {
                let _ = self.chunks[id].send(msg);
            }

            _ => unreachable!(),
        }
    }

    pub fn run(&mut self) {
        let mut next_insert = 0;
        loop {
            match self.core_rx.recv() {
                Ok(msg) => {
                    if let Msg::Shutdown = msg {
                        return ()
                    }
                    self.process_msg(msg, &mut next_insert)
                }

                Err(..) => return (),
            }
        }
    }
}
