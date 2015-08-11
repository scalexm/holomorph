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

        let buf = try!(RawDataMessage {
            content: VarIntVec(chunk.server.patch[0..].to_vec()),
        }.as_packet());

        let _ = self.conn.send(Msg::Write(self.token, buf));
        Ok(())
    }

    pub fn handle_clear_identification(&mut self, _: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        let msg = try!(ClearIdentificationMessage::deserialize(&mut data));
        debug!("{} {}", msg.username, msg.password);

        let mut buf = Vec::new();
        try!(IdentificationSuccessMessage {
            has_rights: Flag(true),
            was_already_connected: Flag(false),
            login: msg.username,
            nickname: "salut".to_string(),
            account_id: 1,
            community_id: 0,
            secret_question: "salut?".to_string(),
            account_creation: 0.,
            subscription_elapsed_duration: 0.,
            subscription_end_date: 0.,
        }.as_packet_with_buf(&mut buf));

        let mut gs = Vec::new();
        gs.push(GameServerInformations {
            id: VarUShort(1),
            status: 3,
            completion: 0,
            is_selectable: true,
            characters_count: 2,
            date: 0.,
        });
        try!(ServersListMessage {
            servers: gs,
            already_connected_to_server_id: VarUShort(0),
            can_create_new_character: true,
        }.as_packet_with_buf(&mut buf));
        
        let _ = self.conn.send(Msg::Write(self.token, buf));

        Ok(())
    }
}
