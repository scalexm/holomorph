mod handlers;

use shared::pool;
use shared::net::{SessionEvent, Token};
use std::io::{self, Cursor};
use std::cell::RefCell;
use server::data::GameServerData;

pub struct Chunk {
    session: Option<RefCell<Session>>,
    server: GameServerData,
}

impl Chunk {
    pub fn new(server: GameServerData) -> Chunk {
        Chunk {
            session: None,
            server: server,
        }
    }

    pub fn process_event(&mut self, evt: SessionEvent) {
        match evt {
            SessionEvent::Connect(tok, addr) => {
                self.session = Some(RefCell::new(<Session as pool::session::Session>
                    ::new(tok, self, addr)));
            }

            SessionEvent::Packet(_, id, data) => {
                let _ = <Session as pool::session::Session>
                    ::handle_packet(&mut self.session.as_ref().unwrap().borrow_mut(),
                        self, id, data);
            }

            SessionEvent::Disconnect(..) => {
                error!("FATAL ERROR: lost connection with auth server");
                self.server.shutdown();
            }
        }
    }
}

impl pool::Chunk for Chunk { }

pub type Sender = pool::Sender<Chunk>;

pub struct Session {
    token: Token,
    address: String,
}

impl pool::session::Session for Session {
    type C = Chunk;

    fn new(token: Token, _: &Chunk, address: String) -> Session {
        Session {
            token: token,
            address: address,
        }
    }

    fn get_handler(id: u16)
        -> (fn(&mut Session, &Chunk, Cursor<Vec<u8>>) -> io::Result<()>) {
        match id {
            1 => Session::handle_hello,
            _ => Session::unhandled,
        }
    }
}
