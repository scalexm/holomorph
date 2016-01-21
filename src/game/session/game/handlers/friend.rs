use session::game::{GameState, Session, SocialState};
use session::game::chunk::Ref;
use std::io::Result;
use protocol::*;
use protocol::messages::game::friend::*;
use protocol::variants::{
    IgnoredInformationsVariant,
    SessionIgnoredInformations
};
use protocol::messages::game::basic::BasicNoOperationMessage;
use server::{social, SERVER};
use character::social::RelationInformations;

impl Session {
    pub fn friend_added_success(&mut self, infos: (i32, RelationInformations)) {
        let (f_id, infos) = (infos.0, infos.1.as_friend());
        let buf = FriendAddedMessage {
            friend_added: infos.clone(),
        }.unwrap();
        write!(SERVER, self.base.token, buf);

        let account = self.account.as_mut().unwrap();
        let _ = account.social.friends.insert(f_id);
        let _ = self.friends_cache.insert(f_id, infos);
    }

    pub fn ignored_added_success(&mut self, infos: (i32, RelationInformations),
                                 for_session: bool) {
        let (i_id, infos) = (infos.0, infos.1.as_ignored());
        let buf = IgnoredAddedMessage {
            ignore_added: infos.clone(),
            session: for_session,
        }.unwrap();
        write!(SERVER, self.base.token, buf);

        if !for_session {
            let account = self.account.as_mut().unwrap();
            let _ = account.social.ignored.insert(i_id);
            let _ = self.ignored_cache.insert(i_id, infos);
        } else {
            let _ = self.ignored_cache.insert(
                i_id,
                IgnoredInformationsVariant::SessionIgnoredInformations(
                    SessionIgnoredInformations {
                        name: infos.name().to_string(),
                    }
                )
            );
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
            friends_list: self.friends_cache.values().cloned().collect(),
        }.unwrap();

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

        let buf = BasicNoOperationMessage.unwrap();
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

        let buf = BasicNoOperationMessage.unwrap();
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
            ignored_list: self.ignored_cache.values().filter_map(|i| {
                match *i {
                    IgnoredInformationsVariant::SessionIgnoredInformations(_) => None,
                    _ => Some(i.clone()),
                }
            }).collect(),
        }.unwrap();

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

         let friends_count = account.social.friends.len();
         if (account.is_subscriber() && friends_count >= 100)
             || (!account.is_subscriber() && friends_count >= 50) {
             let buf = FriendAddFailureMessage {
                 reason: 1,
             }.unwrap();
             write!(SERVER, self.base.token, buf);
             return Ok(());
         }

         let name = msg.name;
         SERVER.with(|s| {
             social::add_relation(
                 &s.server,
                 self.base.token,
                 account.id,
                 name,
                 SocialState::Friend,
                |session, infos| session.friend_added_success(infos)
            );
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
        if !self.friends_cache.contains_key(&msg.account_id) {
            let buf = FriendDeleteResultMessage {
                success: false,
                name: String::new()
            }.unwrap();
            write!(SERVER, self.base.token, buf);
            return Ok(());
        }

        let account_id = {
            let account = self.account.as_mut().unwrap();
            let _ = account.social.friends.remove(&msg.account_id);
            account.id
        };

        let infos = self.friends_cache.remove(&msg.account_id).unwrap();
        let buf = FriendDeleteResultMessage {
            success: true,
            name: infos.name().to_string(),
        }.unwrap();
        write!(SERVER, self.base.token, buf);

        SERVER.with(|s| {
            social::delete_relation(
                &s.server,
                self.base.token,
                account_id,
                msg.account_id,
                SocialState::Friend,
            );
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

         let ignored_count = account.social.ignored.len();
         if (account.is_subscriber() && ignored_count >= 100)
             || (!account.is_subscriber() && ignored_count >= 50) {
             let buf = IgnoredAddFailureMessage {
                 reason: 1,
             }.unwrap();
             write!(SERVER, self.base.token, buf);
             return Ok(());
         }

         let name = msg.name;
         let for_session = msg.session;
         SERVER.with(|s| {
             social::add_relation(
                 &s.server,
                 self.base.token,
                 account.id,
                 name,
                 SocialState::Ignored,
                 move |session, infos| {
                     session.ignored_added_success(infos, for_session)
                 }
             );
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
        if !self.ignored_cache.contains_key(&msg.account_id) {
            let buf = IgnoredDeleteResultMessage {
                success: Flag(false),
                session: Flag(msg.session),
                name: String::new()
            }.unwrap();
            write!(SERVER, self.base.token, buf);
            return Ok(());
        }

        if !msg.session {
            let account = self.account.as_mut().unwrap();
            let _ = account.social.ignored.remove(&msg.account_id);
        }

        let infos = self.ignored_cache.remove(&msg.account_id).unwrap();
        let buf = IgnoredDeleteResultMessage {
            success: Flag(true),
            session: Flag(msg.session),
            name: infos.name().to_string(),
        }.unwrap();
        write!(SERVER, self.base.token, buf);

        SERVER.with(|s| {
            social::delete_relation(
                &s.server,
                self.base.token,
                self.account.as_ref().unwrap().id,
                msg.account_id,
                SocialState::Ignored,
            );
        });
        Ok(())
    }
}
