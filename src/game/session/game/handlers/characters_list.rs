use session::game::{Session, Chunk};
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::messages::game::character::choice::*;
use shared::net::Msg;

impl Session {
    pub fn handle_characters_list_request(&mut self, chunk: &Chunk, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.account.is_none() {
            return Ok(());
        }

        let buf = CharactersListMessage {
            base: BasicCharactersListMessage {
                characters: self.characters
                    .iter()
                    .map(|ch| ch.1.as_character_base())
                    .collect(),
            },
            has_startup_actions: false,
        }.as_packet().unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
        Ok(())
    }
}
