use std::io::{self, Cursor};
use game_session::{Session, Chunk};
use shared::net::Msg;
use shared::protocol::*;
use shared::protocol::holomorph::*;
use crypto::digest::Digest;
use crypto::md5::Md5;
use server;
use shared::pool;

impl Session {
    pub fn start(&self, chunk: &Chunk) {
        let buf = HelloMessage {
            salt: self.salt.clone(),
        }.as_packet().unwrap();

        let _ = chunk.server.game_loop.send(Msg::Write(self.token, buf));
    }

    pub fn handle_identification(&mut self, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.server_id.is_some() {
            return Ok(());
        }

        let msg = try!(IdentificationMessage::deserialize(&mut data));

        let gs = chunk.server.game_servers.get(&msg.id);
        if gs.is_none() {
            let _ = chunk.server.game_loop.send(Msg::Close(self.token));
            return Ok(());
        }

        let gs = gs.unwrap();

        let mut md5 = Md5::new();
        md5.input_str(&gs.key);
        let key = md5.result_str();
        md5 = Md5::new();
        md5.input_str(&(key + &self.salt));
        if md5.result_str() != msg.key {
            let _ = chunk.server.game_loop.send(Msg::Close(self.token));
            return Ok(());
        }

        server::register_game_server(&chunk.server.handler, self.token, msg.id, msg.state,
            |session, chunk, id| session.identification_success(chunk, id));

        Ok(())
    }

    fn identification_success(&mut self, chunk: &Chunk, server_id: Option<i16>) {
        if server_id.is_none() {
            let _ = chunk.server.game_loop.send(Msg::Close(self.token));
            return ();
        }

        self.server_id = server_id;
    }

    pub fn handle_state(&mut self, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.server_id.is_none() {
            return Ok(());
        }

        let msg = try!(StateMessage::deserialize(&mut data));

        let server_id = *self.server_id.as_ref().unwrap();
        let state = msg.state;
        pool::execute(&chunk.server.handler, move |handler|
            handler.update_game_server_status(server_id, state));

        Ok(())
    }
}
