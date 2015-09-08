use session::game::Session;
use session::game::chunk::Ref;
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::messages::game::friend::*;
use server::SERVER;

impl Session {
    pub fn handle_friends_get_list<'a>(&mut self, _: Ref<'a>, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.current_character.is_none() {
            return Ok(())
        }

        let buf = FriendsListMessage {
            friends_list: Vec::new(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_ignored_get_list<'a>(&mut self, _: Ref<'a>, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.current_character.is_none() {
            return Ok(())
        }

        let buf = IgnoredListMessage {
            ignored_list: Vec::new(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);

        Ok(())
    }
}
