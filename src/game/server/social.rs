use super::Server;
use shared::net::Token;
use shared::chunk;
use protocol::variants::{FriendInformationsVariant, IgnoredInformationsVariant,
    PlayerStatusVariant};
use protocol::*;
use protocol::messages::game::friend::*;
use session::game::chunk::SocialState;
use session::game::{self, Session};
use character::CharacterMinimal;
use super::{Sender, SERVER};

impl Server {
    pub fn update_social(&self, ch: &CharacterMinimal, state: SocialState) {
        let social = self.session_socials.get(&ch.account_id()).cloned();
        for chunk in &self.base.main_chunks {
            let ch = ch.clone();
            let social = social.clone();
            chunk::send(chunk, move |chunk| {
                game::chunk::update_social(chunk, ch, social, state);
            });
        }
    }

    pub fn get_friend_infos(&self, f_id: i32, account_id: i32)
                            -> Option<FriendInformationsVariant> {
        self.character_accounts.get(&f_id).map(|ch_id| {
            let ch = self.characters.get(ch_id).unwrap();
            ch.as_friend_infos(account_id, self.session_socials.get(&ch.account_id()))
        })
    }

    pub fn get_ignored_infos(&self, i_id: i32)
                             -> Option<IgnoredInformationsVariant> {
        self.character_accounts.get(&i_id).map(|ch_id| {
            let ch = self.characters.get(ch_id).unwrap();
            ch.as_ignored_infos(self.session_socials.get(&ch.account_id()))
        })
    }

    // return account_id mapping to name
    fn search_for_player(&self, name: &str) -> Option<i32> {
        let name = name.to_lowercase();
        if name.starts_with("*") {
            self.character_nicknames.get(&name[1..])
        } else {
            self.character_names.get(&name).or(self.character_nicknames.get(&name))
        }.map(|id| self.characters.get(&id).unwrap().account_id())
    }
}

pub fn add_friend<F>(sender: &Sender, tok: Token, account_id: i32, name: String, job: F)
                     where F: FnOnce(&mut game::Session, (i32, FriendInformationsVariant))
                     + Send + 'static {
    chunk::send(sender, move |server| {
        if let Some(f_id) = server.search_for_player(&name) {
            if f_id == account_id {
                let buf = FriendAddFailureMessage {
                    reason: 3,
                }.as_packet().unwrap();
                write!(SERVER, tok, buf);
                return;
            }

            let _ = {
                let social = server.session_socials.get_mut(&account_id).unwrap();

                if social.is_friend_with(f_id) {
                    let buf = FriendAddFailureMessage {
                        reason: 4,
                    }.as_packet().unwrap();
                    write!(SERVER, tok, buf);
                    return;
                }

                let _ = social.friends.insert(f_id);
            };

            let ch_id = server.session_characters.inv_get(&tok).unwrap();
            server.update_social(server.characters.get(ch_id).unwrap(),
                                 SocialState::Update);

            let infos = server.get_friend_infos(f_id, account_id).unwrap();
            server.base.session_callback(tok, move |session, _| job(session, (f_id, infos)));
        } else {
            let buf = FriendAddFailureMessage {
                reason: 2,
            }.as_packet().unwrap();
            write!(SERVER, tok, buf);
        }
    });
}

pub fn delete_friend(sender: &Sender, tok: Token, account_id: i32, f_id: i32, f_name: String) {
    chunk::send(sender, move |server| {
        server.session_socials.get_mut(&account_id).unwrap().friends.remove(&f_id);
        let ch_id = server.session_characters.inv_get(&tok).unwrap();
        server.update_social(server.characters.get(ch_id).unwrap(),
                             SocialState::Update);

        let buf = FriendDeleteResultMessage {
            success: true,
            name: f_name,
        }.as_packet().unwrap();
        write!(SERVER, tok, buf);
    })
}

pub fn add_ignored<F>(sender: &Sender, tok: Token, account_id: i32, name: String, job: F)
                      where F: FnOnce(&mut game::Session, (i32, IgnoredInformationsVariant))
                      + Send + 'static {
    chunk::send(sender, move |server| {
        if let Some(i_id) = server.search_for_player(&name) {
            if i_id == account_id {
                let buf = IgnoredAddFailureMessage {
                    reason: 3,
                }.as_packet().unwrap();
                write!(SERVER, tok, buf);
                return;
            }

            let _ = {
                let social = server.session_socials.get_mut(&account_id).unwrap();

                if social.ignores(i_id) {
                    let buf = IgnoredAddFailureMessage {
                        reason: 4,
                    }.as_packet().unwrap();
                    write!(SERVER, tok, buf);
                    return;
                }

                let _ = social.ignored.insert(i_id);
            };

            let ch_id = server.session_characters.inv_get(&tok).unwrap();
            server.update_social(server.characters.get(ch_id).unwrap(),
                                 SocialState::Update);

            let infos = server.get_ignored_infos(i_id).unwrap();
            server.base.session_callback(tok, move |session, _| job(session, (i_id, infos)));
        } else {
            let buf = IgnoredAddFailureMessage {
                reason: 2,
            }.as_packet().unwrap();
            write!(SERVER, tok, buf);
        }
    });
}

pub fn delete_ignored(sender: &Sender, tok: Token, account_id: i32, i_id: i32, i_name: String,
                      session: bool) {
    chunk::send(sender, move |server| {
        server.session_socials.get_mut(&account_id).unwrap().ignored.remove(&i_id);
        let ch_id = server.session_characters.inv_get(&tok).unwrap();
        server.update_social(server.characters.get(ch_id).unwrap(),
                             SocialState::Update);

        let buf = IgnoredDeleteResultMessage {
            success: Flag(true),
            session: Flag(session),
            name: i_name,
        }.as_packet().unwrap();
        write!(SERVER, tok, buf);
    })
}

pub fn update_player_status(sender: &Sender, account_id: i32, ch_id: i32,
                            status: PlayerStatusVariant) {
    chunk::send(sender, move |server| {
        server.session_socials.get_mut(&account_id).unwrap().status = status;
        server.update_social(server.characters.get(&ch_id).unwrap(),
                             SocialState::Update);
    })
}

pub fn update_mood(sender: &Sender, ch_id: i32, mood: i16) {
    chunk::send(sender, move |server| {
        let ch = server.characters.get_mut(&ch_id).unwrap();
        ch.set_mood_smiley(mood);
    })
}
