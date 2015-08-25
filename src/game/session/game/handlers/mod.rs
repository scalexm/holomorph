use super::{Session, Chunk};
use shared::protocol::*;
use shared::protocol::handshake::*;
use shared::protocol::game::approach::*;
use std::io::{self, Cursor};
use shared::net::{Msg, Token};

impl Session {
    pub fn start(&self, chunk: &Chunk) {
        let mut buf = Vec::new();

        ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet_with_buf(&mut buf).unwrap();

        HelloGameMessage.as_packet_with_buf(&mut buf).unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }

    pub fn handle_authentication_ticket(&mut self, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        let msg = try!(AuthenticationTicketMessage::deserialize(&mut data));
        debug!("{}", msg.ticket);

        Ok(())
    }
}
