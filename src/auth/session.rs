use mio;
use std::sync::mpsc::Sender;
use shared::net::{Token, Msg};
use std::io;
use std::io::Cursor;
use shared::io::{ReadExt, WriteExt};
use shared::protocol::*;
use shared::protocol::connection::*;
use shared::protocol::handshake::*;
use shared::pool::session;
use shared::pool;
use chunk::Chunk;

pub struct Session {
    pool: Sender<pool::Msg>,
    conn: mio::Sender<Msg>,
    token: Token,
}

impl Session {
    /*fn load_key() {
        let mut f = File::open("dofus.key").unwrap();
        let mut key = Vec::new();
        f.read_to_end(&mut key).unwrap();
        init_global!(KEY, key);
    }*/

    fn start(&mut self) -> io::Result<()> {

        let mut buf = Vec::new();
        try!(ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet_with_buf(&mut buf));

        try!(HelloConnectMessage {
            salt: "salut".to_string(),
            key: VarIntVec(Vec::new()),
        }.as_packet_with_buf(&mut buf));

        println!("{:?}", buf);
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
        self.conn.send(Msg::Close(self.token)).unwrap()
    }
}

impl session::Session for Session {
    type C = Chunk;

    fn new(token: Token, pool: Sender<pool::Msg>, conn: mio::Sender<Msg>)
        -> Option<Session> {

        debug!("{:?} connected", token);

        let mut s = Session {
            pool: pool,
            conn: conn,
            token: token,
        };

        if let Err(err) = s.start() {
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
            _ => Session::unhandled,
        }
    }
}
