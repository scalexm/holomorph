use super::{Session, SessionImpl};
use super::chunk::Chunk;
use shared::protocol::*;
use shared::protocol::holomorph::*;
use shared::{self, session};
use shared::net::{Token, Msg};
use std::io::{self, Cursor};

impl session::SessionImpl for SessionImpl {
    type Chunk = Chunk;

    fn new(_: Token, _: &Chunk) -> Self {
        SessionImpl
    }

    fn get_handler(id: u16)
        -> (fn(&mut Session, &Chunk, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            1 => handle_hello,
            _ => SessionImpl::unhandled,
        }
    }

    fn close(_: Session, chunk: &Chunk) {
        error!("FATAL ERROR: lost connection with auth server");
        chunk.server.shutdown();
    }
}

pub fn handle_hello(self_: &mut Session, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
    -> io::Result<()> {

    if chunk.connected {
        send!(chunk, Msg::Close(self_.token));
        return Ok(());
    }

    let msg = try!(HelloMessage::deserialize(&mut data));

    let buf = IdentificationMessage {
        id: chunk.server.cnf.server_id,
        key: shared::compute_md5(&(shared::compute_md5(&chunk.server.cnf.server_key)
            + &msg.salt)),
        state: 3,
        ip: chunk.server.cnf.bind_ip.clone(),
        port: chunk.server.cnf.bind_port,
    }.as_packet().unwrap();

    send!(chunk, Msg::Write(self_.token, buf));

    chunk.eventually(|chunk| chunk.connected = true);
    Ok(())
}
