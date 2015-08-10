use shared::net::Msg;
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::connection::*;
use shared::protocol::security::*;
use session::Session;
use chunk::Chunk;

impl Session {
    pub fn handle_identification(&mut self, chunk: &Chunk, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        let msg = try!(RawDataMessage {
            content: VarIntVec(chunk.server.patch[0..].to_vec()),
        }.as_packet());

        let _ = self.conn.send(Msg::Write(self.token, msg));
        Ok(())
    }

    pub fn handle_clear_identification(&mut self, _: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        let msg = try!(ClearIdentificationMessage::deserialize(&mut data));
        debug!("{} {}", msg.username, msg.password);
        Ok(())
    }
}
