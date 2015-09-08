use super::Session;
use super::chunk::{ChunkImpl, Ref};
use shared::protocol::*;
use shared::protocol::holomorph::*;
use shared;
use shared::session::{self, SessionBase};
use std::io::{self, Cursor};
use server::SERVER;

impl session::Session<ChunkImpl> for Session {
    fn new(base: SessionBase) -> Self {
        Session {
            base: base,
        }
    }

    fn get_handler<'a>(id: u16)
        -> (fn(&mut Session, Ref<'a>, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            1 => Session::handle_hello,
            _ => Session::unhandled,
        }
    }

    fn close<'a>(self, _: Ref<'a>) {
        error!("FATAL ERROR: lost connection with auth server");
        SERVER.with(|s| s.shutdown());
    }
}

impl Session {
    fn handle_hello<'a>(&mut self, mut chunk: Ref<'a>, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if chunk.connected {
            close!(SERVER, self.base.token);
            return Ok(());
        }

        let msg = try!(HelloMessage::deserialize(&mut data));

        let md5_key = SERVER.with(|s| shared::compute_md5(&s.cnf.server_key));

        let buf = IdentificationMessage {
            id: SERVER.with(|s| s.cnf.server_id),
            key: shared::compute_md5(&(md5_key + &msg.salt)),
            state: 3,
            ip: SERVER.with(|s| s.cnf.bind_ip.clone()),
            port: SERVER.with(|s| s.cnf.bind_port),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);

        chunk.eventually(|chunk| chunk.impl_.connected = true);
        Ok(())
    }
}
