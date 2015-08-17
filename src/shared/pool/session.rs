use std::io::{self, Cursor};
use pool;
use net::{Msg, Token};
use std::collections::HashMap;
use std::cell::RefCell;

// more specific chunk trait, used for session chunks
pub trait Chunk : pool::Chunk + Sized {
    type S: Session<C = Self>;

    fn sessions(&self) -> &HashMap<Token, RefCell<Self::S>>;
    fn mut_sessions(&mut self) -> &mut HashMap<Token, RefCell<Self::S>>;

    fn process_net_msg(&mut self, msg: Msg) {
        match msg {
            Msg::SessionConnect(tok) => {
                {
                    let sessions = self.sessions();
                    if sessions.contains_key(&tok) {
                        return ();
                    }
                }
                let session = <Self::S as Session>::new(tok, self);
                let _ = self.mut_sessions().insert(tok, RefCell::new(session));
            }

            Msg::SessionDisconnect(tok) => {
                let sessions = self.mut_sessions();
                let _ = sessions.remove(&tok);
            }

            Msg::SessionPacket(tok, id, data) => {
                let sessions = self.sessions();
                if let Some(session) = sessions.get(&tok) {
                    let _ = <Self::S as Session>::handle_packet(&mut session.borrow_mut(),
                        self, id, data);
                }
            }

            _ => unreachable!(),
        }
    }

    fn session_callback<F>(&mut self, tok: Token, job: F)
        where F: FnOnce(&mut Self::S, &Self) {

        if let Some(session) = self.sessions().get(&tok) {
            job(&mut session.borrow_mut(), self)
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
