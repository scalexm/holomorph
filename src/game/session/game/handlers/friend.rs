use session::game::{GameState, Session};
use session::game::chunk::Ref;
use std::io::{Result, Cursor};
use protocol::*;
use protocol::messages::game::friend::*;
use protocol::messages::game::basic::BasicNoOperationMessage;
use server::SERVER;

impl Session {
    pub fn handle_friends_get_list<'a>(&mut self, _: Ref<'a>, _: Cursor<Vec<u8>>) -> Result<()> {
        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        let buf = FriendsListMessage {
            friends_list: self.friends.values().map(|i| i.clone()).collect(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_friend_set_warn_on_connection<'a>(&mut self, _: Ref<'a>,
                                                    mut data: Cursor<Vec<u8>>) -> Result<()> {
        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        let account = self.account.as_mut().unwrap();
        let msg = try!(FriendSetWarnOnConnectionMessage::deserialize(&mut data));
        account.social.warn_on_connection = msg.enable;

        let buf = BasicNoOperationMessage.as_packet().unwrap();
        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_friend_set_warn_on_level_gain<'a>(&mut self, _: Ref<'a>,
                                                    mut data: Cursor<Vec<u8>>) -> Result<()> {
        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        let account = self.account.as_mut().unwrap();
        let msg = try!(FriendSetWarnOnLevelGainMessage::deserialize(&mut data));
        account.social.warn_on_level_gain = msg.enable;

        let buf = BasicNoOperationMessage.as_packet().unwrap();
        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_ignored_get_list<'a>(&mut self, _: Ref<'a>, _: Cursor<Vec<u8>>) -> Result<()> {
        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        let buf = IgnoredListMessage {
            ignored_list: self.ignored.values().map(|i| i.clone()).collect(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);

        Ok(())
    }
}
