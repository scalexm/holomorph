use std::io::{self, Cursor};
use pool;
use net::Token;
use std::collections::HashMap;
use std::cell::RefCell;

pub trait Chunk : pool::Chunk + Sized {
    type S: Session<C = Self>;

    fn sessions(&self) -> &HashMap<Token, RefCell<Self::S>>;
    fn mut_sessions(&mut self) -> &mut HashMap<Token, RefCell<Self::S>>;

    fn process_net_msg(&mut self, msg: pool::NetMsg) {
        match msg {
            pool::NetMsg::SessionConnect(tok) => {
                let session = <Self::S as Session>::new(tok, self);
                let sessions = self.mut_sessions();
                let _ = sessions.insert(tok, RefCell::new(session));
            }

            pool::NetMsg::SessionDisconnect(tok) => {
                let sessions = self.mut_sessions();
                let _ = sessions.remove(&tok);
            }

            pool::NetMsg::SessionPacket(tok, id, data) => {
                let sessions = self.sessions();
                if let Some(session) = sessions.get(&tok) {
                    let _ = <Self::S as Session>::handle_packet(&mut session.borrow_mut(),
                        self, id, data);
                }
            }
        }
    }
}

pub trait Session : Drop {
    type C: Chunk;

    fn new(Token, &Self::C) -> Self;
    fn get_handler(u16) -> (fn(&mut Self, &Self::C, Cursor<Vec<u8>>) -> io::Result<()>);

    fn unhandled(&mut self, _: &Self::C, _: Cursor<Vec<u8>>) -> io::Result<()> {
        Ok(())
    }

    fn handle_packet(&mut self, chunk: &Self::C, id: u16, data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        Self::get_handler(id)(self, chunk, data)
    }
}
