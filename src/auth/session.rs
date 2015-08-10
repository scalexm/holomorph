use mio;
use std::sync::mpsc::Sender;
use shared::net::{Token, Msg};
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::connection::*;
use shared::protocol::handshake::*;
use shared::pool::session;
use chunk::Chunk;

pub struct Session {
    pub conn: mio::Sender<Msg>,
    pub token: Token,
}

impl Session {
    fn start(&mut self, chunk: &Chunk) -> io::Result<()> {

        let mut buf = Vec::new();
        try!(ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet_with_buf(&mut buf));

        try!(HelloConnectMessage {
            salt: "salut".to_string(),
            key: VarIntVec(chunk.key[0..].to_vec()),
        }.as_packet_with_buf(&mut buf));

        if let Err(err) = self.conn.send(Msg::Write(self.token, buf)) {
            error!("notify error: {:?}", err);
            return Err(io::Error::new(io::ErrorKind::Other, "notify error"));
        }
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        debug!("{:?} logout", self.token);
        let _ = self.conn.send(Msg::Close(self.token));
    }
}

impl session::Session for Session {
    type C = Chunk;

    fn new(token: Token, chunk: &Chunk, conn: mio::Sender<Msg>) -> Option<Session> {

        debug!("{:?} connected", token);

        let mut s = Session {
            conn: conn,
            token: token,
        };

        if let Err(err) = s.start(&chunk) {
            error!("error while starting {:?}: {}", token, err);
            error!("{:?} will disconnect", token);
            return None;
        }

        Some(s)
    }

    fn get_handler(id: u16)
        -> (fn(&mut Session, &Chunk, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            4 => Session::handle_identification,
            888 => Session::handle_clear_identification,
            _ => Session::unhandled,
        }
    }
}
