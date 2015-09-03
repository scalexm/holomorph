use session::game::{Session, Chunk};
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::messages::game::friend::*;
use shared::net::Msg;

impl Session {
    pub fn handle_friends_get_list(&mut self, chunk: &Chunk, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.current_character.is_none() {
            return Ok(())
        }

        let buf = FriendsListMessage {
            friends_list: Vec::new(),
        }.as_packet().unwrap();
        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));

        Ok(())
    }

    pub fn handle_ignored_get_list(&mut self, chunk: &Chunk, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.current_character.is_none() {
            return Ok(())
        }

        let buf = IgnoredListMessage {
            ignored_list: Vec::new(),
        }.as_packet().unwrap();
        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));

        Ok(())
    }
}
