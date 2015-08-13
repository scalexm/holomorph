pub mod data;

use shared::pool;
use session::{self, chunk};
use shared::net::{self, Token};
use std::collections::HashMap;
use shared::HashBiMap;

pub type Sender = pool::Sender<Handler>;

pub struct Handler {
    io_loop: net::Sender,
    chunks: Vec<chunk::Sender>,
    session_chunks: HashMap<Token, usize>,
    session_ids: HashBiMap<i32, Token>,
    next_insert: usize,
}

impl Handler {
    pub fn new(io_loop: net::Sender) -> Handler {
        Handler {
            io_loop: io_loop,
            chunks: Vec::new(),
            session_chunks: HashMap::new(),
            next_insert: 0,
            session_ids: HashBiMap::new(),
        }
    }

    fn session_callback<F>(&mut self, tok: Token, job: F)
        where F: FnOnce(&mut session::Session, &chunk::Chunk) + Send + 'static {

        if let Some(chunk) = self.session_chunks.get(&tok) {
            pool::execute(&self.chunks[*chunk], move |chunk| {
                chunk.session_callback(tok, job);
            });
        }
    }
}

pub fn add_chunk(sender: &Sender, chunk: chunk::Sender) {
    pool::execute(sender, move |handler| {
        handler.chunks.push(chunk)
    });
}

pub fn identification_success<F>(sender: &Sender, tok: Token, id: i32, job: F)
    where F: FnOnce(&mut session::Session, &chunk::Chunk, bool) + Send + 'static {

    pool::execute(sender, move |handler| {
        let already = handler.session_ids.insert(id, tok);
        if let Some(session) = already {
            let _ = handler.io_loop.send(net::Msg::Close(session));
        }

        handler.session_callback(tok,
            move |session: &mut session::Session, chunk: &chunk::Chunk|
                job(session, chunk, already.is_some()))
    });
}

pub fn session_callback<F>(sender: &Sender, tok: Token, job: F)
    where F: FnOnce(&mut session::Session, &chunk::Chunk) + Send + 'static {

    pool::execute(sender, move |handler| {
        handler.session_callback(tok, job)
    });
}

impl pool::Chunk for Handler {
    fn process_net_msg(&mut self, msg: pool::NetMsg) {
        match msg {
            pool::NetMsg::SessionConnect(tok) => {
                if self.session_chunks.contains_key(&tok) {
                    return ();
                }

                let chunk = self.next_insert % self.chunks.len();
                let _ = self.session_chunks.insert(tok, chunk);
                let _  = self.chunks[chunk].send(msg.into());

                self.next_insert += 1;
            }

            pool::NetMsg::SessionPacket(tok, _, _) => {
                if let Some(chunk) = self.session_chunks.get(&tok) {
                    let _ = self.chunks[*chunk].send(msg.into());
                }
            }

            pool::NetMsg::SessionDisconnect(tok) => {
                if let Some(chunk) = self.session_chunks.get(&tok) {
                    let _ = self.chunks[*chunk].send(msg.into());
                }
                let _ = self.session_chunks.remove(&tok);
                let _ = self.session_ids.inv_remove(&tok);
            }
        }
    }
}
