pub mod data;

use session::{auth, game};
use std::collections::HashMap;
use shared::net::{self, Token, SessionEvent};
use shared::pool;
use shared::HashBiMap;

pub type Sender = pool::Sender<Handler>;

pub struct Handler {
    io_loop: net::Sender,
    chunks: Vec<game::Sender>,
    auth_chunk: Option<auth::Sender>,
    session_chunks: HashMap<Token, usize>,
    session_ids: HashBiMap<i32, Token>,
    next_insert: usize,
}

impl pool::Chunk for Handler { }

impl Handler {
    pub fn new(io_loop: net::Sender) -> Handler {
        Handler {
            io_loop: io_loop,
            chunks: Vec::new(),
            auth_chunk: None,
            session_chunks: HashMap::new(),
            next_insert: 0,
            session_ids: HashBiMap::new(),
        }
    }
}

pub fn add_chunk(sender: &Sender, chunk: game::Sender) {
    pool::execute(sender, move |handler| {
        handler.chunks.push(chunk)
    });
}

pub fn set_auth_chunk(sender: &Sender, chunk: auth::Sender) {
    pool::execute(sender, move |handler| {
        handler.auth_chunk = Some(chunk);
    });
}

// handling session events from NetworkHandler
impl Handler {
    pub fn auth_event(&mut self, evt: SessionEvent) {
       pool::execute(self.auth_chunk.as_ref().unwrap(), move |chunk| {
           chunk.process_event(evt);
       });
   }

    pub fn game_event(&mut self, evt: SessionEvent) {
        use shared::pool::session::Chunk;

        match evt {
            SessionEvent::Connect(tok, _) => {
                if self.session_chunks.contains_key(&tok) {
                    return ();
                }

                let chunk = self.next_insert % self.chunks.len();
                let _ = self.session_chunks.insert(tok, chunk);
                pool::execute(&self.chunks[chunk], move |chunk| {
                    chunk.process_event(evt);
                });

                self.next_insert += 1;
            }

            SessionEvent::Packet(tok, _, _) => {
                if let Some(chunk) = self.session_chunks.get(&tok) {
                    pool::execute(&self.chunks[*chunk], move |chunk| {
                        chunk.process_event(evt);
                    });
                }
            }

            SessionEvent::Disconnect(tok) => {
                if let Some(chunk) = self.session_chunks.get(&tok) {
                    pool::execute(&self.chunks[*chunk], move |chunk| {
                        chunk.process_event(evt);
                    });
                }
                let _ = self.session_chunks.remove(&tok);
                let _ = self.session_ids.inv_remove(&tok);
            }
        }
    }
}
