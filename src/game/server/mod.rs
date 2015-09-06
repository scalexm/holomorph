pub mod data;

use session::{auth, game};
use std::collections::HashMap;
use shared::net::{self, Token, SessionEvent};
use shared::chunk;
use shared::HashBiMap;
use eventual::{Timer, Async};
use character::CharacterMinimal;

pub type Sender = chunk::Sender<Handler>;

pub struct Handler {
    io_loop: net::Sender,
    chunks: Vec<game::chunk::Sender>,
    auth_chunk: Option<auth::chunk::Sender>,
    session_chunks: HashMap<Token, usize>,

    // a session can be identified by its account id
    session_ids: HashBiMap<i32, Token>,

    // an in-game session can be identified by its character id
    session_characters: HashBiMap<i32, Token>,

    next_insert: usize,
    queue_timer: Timer,
    characters: HashMap<i32, CharacterMinimal>,
}

impl Handler {
    pub fn new(io_loop: net::Sender) -> Self {
        Handler {
            io_loop: io_loop,
            chunks: Vec::new(),
            auth_chunk: None,
            session_chunks: HashMap::new(),
            next_insert: 0,
            session_ids: HashBiMap::new(),
            session_characters: HashBiMap::new(),
            queue_timer: Timer::with_capacity(1),
            characters: HashMap::new(),
        }
    }

    fn session_callback<F>(&self, tok: Token, job: F)
        where F: FnOnce(&mut game::Session, &game::chunk::Chunk) + Send + 'static {

        if let Some(chunk) = self.session_chunks.get(&tok) {
            chunk::send(&self.chunks[*chunk], move |chunk| {
                chunk.session_callback(tok, job)
            });
        }
    }
}

pub fn start_queue_timer(sender: &Sender) {
    let tx = sender.clone();
    chunk::send(sender, move |handler| {
        handler.queue_timer.interval_ms(2000).each(move |()| {
            chunk::send(&tx, move |handler| {
                for chunk in &handler.chunks {
                    chunk::send(chunk, |chunk| {
                        game::chunk::update_queue(chunk);
                    });
                }
            })
        }).fire();
    });
}

pub fn add_chunk(sender: &Sender, chunk: game::chunk::Sender) {
    chunk::send(sender, move |handler| {
        handler.chunks.push(chunk)
    });
}

pub fn set_auth_chunk(sender: &Sender, chunk: auth::chunk::Sender) {
    chunk::send(sender, move |handler| {
        handler.auth_chunk = Some(chunk);
    });
}

pub fn identification_success<F>(sender: &Sender, tok: Token, id: i32, job: F)
    where F: FnOnce(&mut game::Session, &game::chunk::Chunk, bool,
        HashMap<i32, CharacterMinimal>) + Send + 'static {

    chunk::send(sender, move |handler| {
        let already = handler.session_ids.contains_key(&id);
        let mut characters = HashMap::new();

        if !already {
            let _ = handler.session_ids.insert(id, tok);

            characters = handler.characters.iter().filter_map(|ch| {
                if ch.1.account_id() == id {
                    return Some((*ch.0, ch.1.clone()));
                }
                None
            }).collect();
        }

        handler.session_callback(tok,
            move |session, chunk|
                job(session, chunk, already, characters))
    });
}

pub fn character_selection_success<F>(sender: &Sender, tok: Token, ch_id: i32,
    job: F) where F: FnOnce(&mut game::Session, &game::chunk::Chunk) + Send + 'static {

    chunk::send(sender, move |handler| {
        let _ = handler.session_characters.insert(ch_id, tok);

        handler.session_callback(tok,
            move |session, chunk|
                job(session, chunk))
    });
}

// handling session events from NetworkHandler
impl Handler {
    pub fn auth_event(&mut self, evt: SessionEvent) {
       chunk::send(self.auth_chunk.as_ref().unwrap(), move |chunk| {
           chunk.process_event(evt);
       });
   }

    pub fn game_event(&mut self, evt: SessionEvent) {
        match evt {
            SessionEvent::Connect(tok, _) => {
                if self.session_chunks.contains_key(&tok) {
                    return ();
                }

                let chunk = self.next_insert % self.chunks.len();
                let _ = self.session_chunks.insert(tok, chunk);
                chunk::send(&self.chunks[chunk], move |chunk| {
                    chunk.process_event(evt);
                });

                self.next_insert += 1;
            }

            SessionEvent::Packet(tok, _, _) => {
                if let Some(chunk) = self.session_chunks.get(&tok) {
                    chunk::send(&self.chunks[*chunk], move |chunk| {
                        chunk.process_event(evt);
                    });
                }
            }

            SessionEvent::Disconnect(tok) => {
                if let Some(chunk) = self.session_chunks.get(&tok) {
                    chunk::send(&self.chunks[*chunk], move |chunk| {
                        chunk.process_event(evt);
                    });
                }
                let _ = self.session_chunks.remove(&tok);
                let _ = self.session_ids.inv_remove(&tok);
                let _ = self.session_characters.inv_remove(&tok);
            }
        }
    }
}
