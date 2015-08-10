pub mod chunk;
pub mod session;

use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use net::{self, Token};
use std::io::Cursor;
use std::collections::HashMap;
use mio;
use self::chunk::Chunk;

pub enum Msg {
    Shutdown,
    SessionCreate(Token, mio::Sender<net::Msg>),
    SessionPacket(Token, u16, Cursor<Vec<u8>>),
    SessionRemove(Token),
}

pub struct SessionPool<C: Chunk> {
    core_tx: Sender<Msg>,
    core_rx: Receiver<Msg>,
    chunks: Vec<Sender<chunk::Msg<C>>>,
    session_map: HashMap<Token, usize>,
}

impl<C: Chunk + Send + 'static> SessionPool<C> {
    pub fn run_chunk(&mut self, mut chunk: C) -> usize {
        let (tx, rx) = channel::<chunk::Msg<C>>();
        let id = self.chunks.len();
        self.chunks.push(tx);

        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(msg) => chunk.process_msg(msg),
                    Err(..) => return (),
                }
            }
        });

        id
    }

    pub fn new() -> SessionPool<C> {
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
            Msg::SessionCreate(tok, _) => {
                if self.session_map.contains_key(&tok) {
                    return ();
                }

                let chunk = *next_insert % self.chunks.len();
                let _ = self.session_map.insert(tok, chunk);
                self.chunks[chunk]
                    .send(chunk::Msg::PoolMsg(msg))
                    .unwrap();

                *next_insert += 1;
            }

            Msg::SessionPacket(tok, _, _) => {
                if let Some(chunk) = self.session_map.get(&tok) {
                    self.chunks[*chunk].send(chunk::Msg::PoolMsg(msg)).unwrap();
                }
            }

            Msg::SessionRemove(tok) => {
                if let Some(chunk) = self.session_map.get(&tok) {
                    self.chunks[*chunk].send(chunk::Msg::PoolMsg(msg)).unwrap();
                }
                let _ = self.session_map.remove(&tok);
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
                        return ();
                    }
                    self.process_msg(msg, &mut next_insert)
                }

                Err(..) => panic!("core_rx.recv failed"),
            }
        }
    }
}
