use chunk::{self, Sender};
use session::chunk::{Chunk, Ref};
use session::Session;
use eventual::Timer;
use HashBiMap;
use std::collections::HashMap;
use net::{Token, SessionEvent};

pub struct ServerBase<T1: Session<U1>, U1, T2: Session<U2>, U2> {
    pub main_chunks: Vec<Sender<Chunk<T1, U1>>>,
    pub secondary_chunk: Option<Sender<Chunk<T2, U2>>>,
    pub session_chunks: HashMap<Token, usize>,
    pub session_ids: HashBiMap<i32, Token>,
    next_insert: usize,
    pub timer: Timer,
}

impl<T1: Session<U1>, U1, T2: Session<U2>, U2> ServerBase<T1, U1, T2, U2> {
    pub fn new() -> Self{
        ServerBase {
            main_chunks: Vec::new(),
            secondary_chunk: None,
            session_chunks: HashMap::new(),
            session_ids: HashBiMap::new(),
            next_insert: 0,
            timer: Timer::with_capacity(1),
        }
    }

    pub fn session_callback<F>(&self, tok: Token, job: F)
                           where F: for<'a> FnOnce(&mut T1, Ref<'a, T1, U1>) + Send + 'static {

        if let Some(chunk) = self.session_chunks.get(&tok) {
            chunk::send(&self.main_chunks[*chunk], move |chunk| {
                chunk.session_callback(tok, job)
            });
        }
    }

    pub fn main_event(&mut self, evt: SessionEvent) {
        match evt {
            SessionEvent::Connect(tok, _) => {
                if self.session_chunks.contains_key(&tok) {
                    return;
                }

                let chunk = self.next_insert % self.main_chunks.len();
                let _ = self.session_chunks.insert(tok, chunk);
                chunk::send(&self.main_chunks[chunk], move |chunk| {
                    chunk.process_event(evt);
                });

                self.next_insert += 1;
            }

            SessionEvent::Packet(tok, _, _) => {
                if let Some(chunk) = self.session_chunks.get(&tok) {
                    chunk::send(&self.main_chunks[*chunk], move |chunk| {
                        chunk.process_event(evt);
                    });
                }
            }

            SessionEvent::Disconnect(tok) => {
                if let Some(chunk) = self.session_chunks.get(&tok) {
                    chunk::send(&self.main_chunks[*chunk], move |chunk| {
                        chunk.process_event(evt);
                    });
                }
                let _ = self.session_chunks.remove(&tok);
                let _ = self.session_ids.inv_remove(&tok);
            }
        }
    }

    pub fn secondary_event(&mut self, evt: SessionEvent) {
       chunk::send(self.secondary_chunk.as_ref().unwrap(), move |chunk| {
           chunk.process_event(evt);
       });
   }
}
