use shared::pool::{self, chunk, session};
use session::Session;
use shared::net::Token;
use std::cell::RefCell;
use std::sync::mpsc::Sender;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Chunk {
    sessions: HashMap<Token, RefCell<Session>>,
    pool: Sender<pool::Msg>,
    pub key: Arc<Vec<u8>>,
    pub patch: Arc<Vec<u8>>,
}

impl Chunk {
    pub fn new(pool: Sender<pool::Msg>, key: Arc<Vec<u8>>, patch: Arc<Vec<u8>>) -> Chunk {
        Chunk {
            sessions: HashMap::new(),
            pool: pool,
            key: key,
            patch: patch,
        }
    }

    fn process_pool_msg(&mut self, msg: pool::Msg) {
        match msg {
            pool::Msg::SessionCreate(tok, snd) => {
                if let Some(session) = <Session as session::Session>
                    ::new(tok, self, snd) {

                    let _ = self.sessions.insert(tok, RefCell::new(session));
                }
            }

            pool::Msg::SessionRemove(tok) => {
                let _ = self.sessions.remove(&tok);
            }

            pool::Msg::SessionPacket(tok, id, data) => {
                if let Some(session) = self.sessions.get(&tok) {
                    <Session as session::Session>
                        ::handle_packet(&mut session.borrow_mut(),
                            self, id, data).unwrap();
                }
            }

            _ => unreachable!(),
        }
    }
}

impl chunk::Chunk for Chunk {
    type Msg = ();

    fn process_msg(&mut self, msg: chunk::Msg<Chunk>) {
        match msg {
            chunk::Msg::PoolMsg(msg) => self.process_pool_msg(msg),
            _ => (),
        }
    }
}
