use mio::{Token, Sender};
use shared::net::server::Msg;
use std::io;
use std::io::Cursor;
use shared::io::{ReadExt, WriteExt};

pub struct Session {
    sender: Sender<Msg>,
    token: Token,
}

impl ::shared::Session for Session {
    fn new(token: Token, sender: Sender<Msg>) -> Session {
        Session {
            sender: sender,
            token: token,
        }
    }

    fn get_handler(id: u16) -> (fn(&mut Session, Cursor<Vec<u8>>) -> io::Result<()>) {
        match id {
            12 => Session::handle_test,
            _ => Session::unhandled,
        }
    }
}

impl Session {
    fn handle_test(&mut self, mut data: Cursor<Vec<u8>>) -> io::Result<()> {
        println!("{}", data.read_string().unwrap());

        let mut w = Vec::new();
        w.write_string("yo ma gueule");
        let mut buf = Vec::new();
        buf.write_packet(10, &w);
        self.sender.send(Msg::Write(self.token, buf)).unwrap();
        Ok(())
    }
}
