use shared::pool::{self, session};
use session::Session;
use shared::net::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use server::AuthServer;

pub struct Chunk {
    sessions: HashMap<Token, RefCell<Session>>,
    pub server: AuthServer,
}

impl Chunk {
    pub fn new(server: AuthServer) -> Chunk {

        Chunk {
            sessions: HashMap::new(),
            server: server,
        }
    }
}

impl pool::Chunk for Chunk {
    fn process_msg(&mut self, msg: pool::Msg) {
        match msg {
            pool::Msg::SessionCreate(tok) => {
                if let Some(session) = Session::new(tok, self,
                    self.server.io_loop.clone()) {

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

            pool::Msg::SessionCallback(tok, cb) => {
                if let Some(session) = self.sessions.get(&tok) {
                    use std::ops::DerefMut;
                    cb.call_box(session.borrow_mut().deref_mut(), self);
                }
            }

            pool::Msg::ChunkCallback(_, cb) => {
                cb.call_box(&mut (), self);
            }

            _ => unreachable!(),
        }
    }
}
