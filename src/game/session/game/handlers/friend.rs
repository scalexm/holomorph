use session::game::{GameState, Session};
use session::game::chunk::Ref;
use std::io::{Result, Cursor};
use protocol::*;
use protocol::messages::game::friend::*;
use protocol::variants::{FriendInformationsVariant, IgnoredInformationsVariant,
    SessionIgnoredInformations};
use protocol::messages::game::basic::BasicNoOperationMessage;
use server::{social, SERVER};

impl Session {
    pub fn friend_added_success(&mut self, infos: (i32, FriendInformationsVariant)) {
        let buf = FriendAddedMessage {
            friend_added: infos.1.clone(),
        }.as_packet().unwrap();
        write!(SERVER, self.base.token, buf);

        let _ = self.account.as_mut().unwrap().social.friends.insert(infos.0);
        let _ = self.friends.insert(infos.0, infos.1);
    }

    pub fn ignored_added_success(&mut self, infos: (i32, IgnoredInformationsVariant),
                                 for_session: bool) {
        let buf = IgnoredAddedMessage {
            ignore_added: infos.1.clone(),
            session: for_session,
        }.as_packet().unwrap();
        write!(SERVER, self.base.token, buf);

        if !for_session {
            let _ = self.account.as_mut().unwrap().social.ignored.insert(infos.0);
            let _ = self.ignored.insert(infos.0, infos.1);
        } else {
            let _ = self.ignored.insert(infos.0, IgnoredInformationsVariant
                ::SessionIgnoredInformations(SessionIgnoredInformations {
                    name: infos.1.name().to_string(),
                }));
        }
    }
}

#[register_handlers]
impl Session {
    pub fn handle_friends_get_list<'a>(&mut self, _: Ref<'a>, _: FriendsGetListMessage)
                                       -> Result<()> {
        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        let buf = FriendsListMessage {
            friends_list: self.friends.values().cloned().collect(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_friend_set_warn_on_connection<'a>(&mut self, _: Ref<'a>,
                                                    msg: FriendSetWarnOnConnectionMessage)
                                                    -> Result<()> {
        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        let account = self.account.as_mut().unwrap();

        account.social.warn_on_connection = msg.enable;

        let buf = BasicNoOperationMessage.as_packet().unwrap();
        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_friend_set_warn_on_level_gain<'a>(&mut self, _: Ref<'a>,
                                                    msg: FriendSetWarnOnLevelGainMessage)
                                                    -> Result<()> {
        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        let account = self.account.as_mut().unwrap();

        account.social.warn_on_level_gain = msg.enable;

        let buf = BasicNoOperationMessage.as_packet().unwrap();
        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_ignored_get_list<'a>(&mut self, _: Ref<'a>, _: IgnoredGetListMessage)
                                       -> Result<()> {
        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        let buf = IgnoredListMessage {
            ignored_list: self.ignored.values().filter_map(|i| {
                match *i {
                    IgnoredInformationsVariant::SessionIgnoredInformations(_) => None,
                    _ => Some(i.clone()),
                }
            }).collect(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_friend_add_request<'a>(&mut self, _: Ref<'a>, msg: FriendAddRequestMessage)
                                         -> Result<()> {
         match self.state {
             GameState::InContext(_) => (),
             _ => return Ok(()),
         };

         let account = self.account.as_ref().unwrap();

         if (account.is_subscriber() && account.social.friends.len() >= 100)
             || (!account.is_subscriber() && account.social.friends.len() >= 50) {
             let buf = FriendAddFailureMessage {
                 reason: 1,
             }.as_packet().unwrap();
             write!(SERVER, self.base.token, buf);
             return Ok(());
         }

         let name = msg.name;
         SERVER.with(|s| {
             social::add_friend(&s.server, self.base.token, account.id, name,
                               |session, infos| session.friend_added_success(infos));
        });
         Ok(())
    }

    pub fn handle_friend_delete_request<'a>(&mut self, _: Ref<'a>,
                                            msg: FriendDeleteRequestMessage) -> Result<()> {
        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        // do not call account.social.is_friend_with because we can only delete friends
        // which are on the server
        if !self.friends.contains_key(&msg.account_id) {
            let buf = FriendDeleteResultMessage {
                success: false,
                name: String::new()
            }.as_packet().unwrap();
            write!(SERVER, self.base.token, buf);
            return Ok(());
        }

        let _ = self.account.as_mut().unwrap().social.friends.remove(&msg.account_id);
        let infos = self.friends.remove(&msg.account_id).unwrap();
        SERVER.with(|s| {
            social::delete_friend(&s.server, self.base.token, self.account.as_ref().unwrap().id,
                                  msg.account_id, infos.name().to_string());
        });
        Ok(())
    }

    pub fn handle_ignored_add_request<'a>(&mut self, _: Ref<'a>, msg: IgnoredAddRequestMessage)
                                         -> Result<()> {
         match self.state {
             GameState::InContext(_) => (),
             _ => return Ok(()),
         };

         let account = self.account.as_ref().unwrap();

         if (account.is_subscriber() && account.social.ignored.len() >= 100)
             || (!account.is_subscriber() && account.social.ignored.len() >= 50) {
             let buf = IgnoredAddFailureMessage {
                 reason: 1,
             }.as_packet().unwrap();
             write!(SERVER, self.base.token, buf);
             return Ok(());
         }

         let name = msg.name;
         let for_session = msg.session;
         SERVER.with(|s| {
             social::add_ignored(&s.server, self.base.token, account.id, name,
                               move |session, infos| {
                                   session.ignored_added_success(infos, for_session)
                               });
        });
         Ok(())
    }

    pub fn handle_ignored_delete_request<'a>(&mut self, _: Ref<'a>,
                                             msg: IgnoredDeleteRequestMessage) -> Result<()> {
        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        // same as in handle_friend_delete_request
        if !self.ignored.contains_key(&msg.account_id) {
            let buf = IgnoredDeleteResultMessage {
                success: Flag(false),
                session: Flag(msg.session),
                name: String::new()
            }.as_packet().unwrap();
            write!(SERVER, self.base.token, buf);
            return Ok(());
        }

        if !msg.session {
            let _ = self.account.as_mut().unwrap().social.ignored.remove(&msg.account_id);
        }
        let infos = self.ignored.remove(&msg.account_id).unwrap();
        SERVER.with(|s| {
            social::delete_ignored(&s.server, self.base.token, self.account.as_ref().unwrap().id,
                                   msg.account_id, infos.name().to_string(), msg.session);
        });
        Ok(())
    }
}
