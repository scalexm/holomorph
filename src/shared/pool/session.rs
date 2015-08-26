use std::io::{self, Cursor};
use pool;
use net::{SessionEvent, Token};
use std::collections::HashMap;
use std::cell::RefCell;

// more specific chunk trait, used for session chunks
pub trait Chunk : pool::Chunk + Sized {
    type S: Session<C = Self>;

    fn sessions(&self) -> &HashMap<Token, RefCell<Self::S>>;
    fn mut_sessions(&mut self) -> &mut HashMap<Token, RefCell<Self::S>>;

    fn process_event(&mut self, evt: SessionEvent) {
        match evt {
            SessionEvent::Connect(tok, addr) => {
                {
                    let sessions = self.sessions();
                    if sessions.contains_key(&tok) {
                        return ();
                    }
                }
                let session = <Self::S as Session>::new(tok, self, addr);
                let _ = self.mut_sessions().insert(tok, RefCell::new(session));
            }

            SessionEvent::Disconnect(tok) => {
                let session = self.mut_sessions().remove(&tok);
                if let Some(session) = session {
                    session.into_inner().close(self);
                }
            }

            SessionEvent::Packet(tok, id, data) => {
                let sessions = self.sessions();
                if let Some(session) = sessions.get(&tok) {
                    debug!("{:?} received a packet, id {}", tok, id);
                    if let Err(err) = <Self::S as Session>
                        ::handle_packet(&mut session.borrow_mut(), self, id, data) {

                        /* debug only because it means that some message::Deserialize failed,
                         so it is either an issue with the client or an issue with Deserialize */
                        debug!("handle_packet io error: {}, id {}", err, id);
                    }
                }
            }
        }
    }

    fn session_callback<F>(&mut self, tok: Token, job: F)
        where F: FnOnce(&mut Self::S, &Self) {

        if let Some(session) = self.sessions().get(&tok) {
            job(&mut session.borrow_mut(), self)
        }
    }
}

pub trait Session: Sized {
    type C;

    fn new(Token, &Self::C, String) -> Self;
    fn get_handler(u16) -> (fn(&mut Self, &Self::C, Cursor<Vec<u8>>) -> io::Result<()>);

    fn unhandled(&mut self, _: &Self::C, _: Cursor<Vec<u8>>) -> io::Result<()> {
        Ok(())
    }

    fn handle_packet(&mut self, chunk: &Self::C, id: u16, data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        Self::get_handler(id)(self, chunk, data)
    }

    fn close(self, _: &Self::C) { }
}
