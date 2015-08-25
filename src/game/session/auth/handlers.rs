use super::{Session, Chunk};
use shared::protocol::*;
use shared::protocol::holomorph::*;
use shared;
use shared::net::Msg;
use std::io::{self, Cursor};

impl Session {
    pub fn handle_hello(&mut self, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        let msg = try!(HelloMessage::deserialize(&mut data));

        let buf = IdentificationMessage {
            id: chunk.server.cnf.server_id,
            key: shared::compute_md5(&(shared::compute_md5(&chunk.server.cnf.server_key)
                + &msg.salt)),
            state: 3,
            ip: chunk.server.cnf.bind_ip.clone(),
            port: chunk.server.cnf.bind_port,
        }.as_packet().unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
        Ok(())
    }
}
