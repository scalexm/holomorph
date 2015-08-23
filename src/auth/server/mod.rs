pub mod data;

use shared::pool;
use session::{auth, game};
use shared::net::{self, Token, SessionEvent};
use std::collections::HashMap;
use shared::HashBiMap;
use eventual::{Timer, Async};
use shared::protocol::Protocol;
use shared::protocol::connection::server_status;
use shared::protocol::holomorph::DisconnectPlayerMessage;

pub type Sender = pool::Sender<Handler>;

pub struct Handler {
    io_loop: net::Sender,
    chunks: Vec<auth::Sender>,
    game_chunk: Option<game::Sender>,
    session_chunks: HashMap<Token, usize>,
    session_ids: HashBiMap<i32, Token>,
    game_session_ids: HashBiMap<i16, Token>,
    next_insert: usize,
    queue_timer: Timer,
}

impl pool::Chunk for Handler { }

impl Handler {
    pub fn new(io_loop: net::Sender) -> Handler {
        Handler {
            io_loop: io_loop,
            chunks: Vec::new(),
            game_chunk: None,
            session_chunks: HashMap::new(),
            next_insert: 0,
            session_ids: HashBiMap::new(),
            game_session_ids: HashBiMap::new(),
            queue_timer: Timer::with_capacity(1),
        }
    }

    fn session_callback<F>(&self, tok: Token, job: F)
        where F: FnOnce(&mut auth::Session, &auth::Chunk) + Send + 'static {

        if let Some(chunk) = self.session_chunks.get(&tok) {
            pool::execute(&self.chunks[*chunk], move |chunk| {
                use shared::pool::session::Chunk;
                chunk.session_callback(tok, job)
            });
        }
    }

    fn update_game_server(&mut self, id: i16, status: i8,
        ip: String, port: u16) {

        for chunk in &self.chunks {
            let ip = ip.clone();
            pool::execute(chunk, move |chunk| {
                chunk.update_game_server(id, status, ip, port);
            });
        }
    }
}

pub fn start_queue_timer(sender: &Sender) {
    let tx = sender.clone();
    pool::execute(sender, move |handler| {
        handler.queue_timer.interval_ms(2000).each(move |()| {
            pool::execute(&tx, move |handler| {
                for chunk in &handler.chunks {
                    pool::execute(chunk, |chunk| {
                        chunk.update_queue();
                    });
                }
            })
        }).fire();
    });
}

pub fn add_chunk(sender: &Sender, chunk: auth::Sender) {
    pool::execute(sender, move |handler| {
        handler.chunks.push(chunk)
    });
}

pub fn set_game_chunk(sender: &Sender, chunk: game::Sender) {
    pool::execute(sender, move |handler| {
        handler.game_chunk = Some(chunk);
    });
}

pub fn identification_success<F>(sender: &Sender, tok: Token, id: i32,
    already_logged: i16, job: F)
    where F: FnOnce(&mut auth::Session, &auth::Chunk, bool)
    + Send + 'static {

    pool::execute(sender, move |handler| {
        let already = handler.session_ids.insert(id, tok);
        if let Some(session) = already {
            let _ = handler.io_loop.send(net::Msg::Close(session));
        }
        if let Some(tok) = handler.game_session_ids.get(&already_logged) {
            let buf = DisconnectPlayerMessage {
                id: id,
            }.as_packet().unwrap();
            let _ = handler.io_loop.send(net::Msg::Write(*tok, buf));
        }

        handler.session_callback(tok,
            move |session, chunk|
                job(session, chunk, already.is_some()))
    });
}

pub fn register_game_server<F>(sender: &Sender, tok: Token, id: i16, state: i8,
    ip: String, port: u16, job: F)
    where F: FnOnce(&mut game::Session, &game::Chunk, Option<i16>)
    + Send + 'static {

    pool::execute(sender, move |handler| {
        let mut server_id = None;
        if !handler.game_session_ids.contains_key(&id) {
            let _ = handler.game_session_ids.insert(id, tok);
            handler.update_game_server(id, state, ip, port);
            server_id = Some(id);
        }

        pool::execute(handler.game_chunk.as_ref().unwrap(), move |chunk| {
            use shared::pool::session::Chunk;
            chunk.session_callback(tok, move |session, chunk| {
                job(session, chunk, server_id)
            });
        });
    });
}

pub fn update_game_server(sender: &Sender, id: i16, state: i8, ip: String, port: u16) {
    pool::execute(sender, move |handler| {
        handler.update_game_server(id, state, ip, port)
    });
}

// handling session events from NetworkHandler
impl Handler {
    pub fn game_event(&mut self, evt: SessionEvent) {
        use shared::pool::session::Chunk;

        if let SessionEvent::Disconnect(tok) = evt {
           let id = self.game_session_ids.inv_remove(&tok);
           if id.is_some() {
               self.update_game_server(id.unwrap(),
                   server_status::OFFLINE, "".to_string(), 0);
           }
       }

       pool::execute(self.game_chunk.as_ref().unwrap(), move |chunk| {
           chunk.process_event(evt);
       });
   }

    pub fn auth_event(&mut self, evt: SessionEvent) {
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
