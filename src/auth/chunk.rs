use shared::pool::chunk;
use shared::pool;
use session::Session;
use shared::pool::session;
use shared::net::Token;
use std::cell::RefCell;
use std::sync::mpsc::Sender;
use std::collections::HashMap;

pub struct Chunk {
    sessions: HashMap<Token, RefCell<Session>>,
    pool: Sender<pool::Msg>,
}

impl Chunk {
    pub fn new(pool: Sender<pool::Msg>) -> Chunk {
        Chunk {
            sessions: HashMap::new(),
            pool: pool,
        }
    }

    fn process_pool_msg(&mut self, msg: pool::Msg) {
        match msg {
            pool::Msg::SessionCreate(tok, snd) => {
                if let Some(session) = <Session as session::Session>
                    ::new(tok, self.pool.clone(), snd) {

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
