use session::game::Session;
use session::game::chunk::Ref;
use std::io::{Result, Cursor};
use shared::protocol::*;
use shared::protocol::messages::game::friend::*;
use server::SERVER;

impl Session {
    pub fn handle_friends_get_list<'a>(&mut self, _: Ref<'a>, _: Cursor<Vec<u8>>) -> Result<()> {
        let buf = FriendsListMessage {
            friends_list: self.friends.values().map(|i| i.clone()).collect(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_ignored_get_list<'a>(&mut self, _: Ref<'a>, _: Cursor<Vec<u8>>) -> Result<()> {
        let buf = IgnoredListMessage {
            ignored_list: self.ignored.values().map(|i| i.clone()).collect(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);

        Ok(())
    }
}
