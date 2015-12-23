use super::Server;
use shared::net::Token;
use shared::chunk;
use protocol::variants::PlayerStatusVariant;
use protocol::*;
use protocol::messages::game::friend::*;
use session::game::chunk::SocialUpdateType;
use session::game::{self, Session, SocialState};
use character::CharacterMinimal;
use character::social::RelationInformations;
use super::{Sender, SERVER};

impl Server {
    pub fn update_social(&self, ch: &CharacterMinimal, ty: SocialUpdateType) {
        let social = self.session_socials.get(&ch.account_id()).cloned();
        for chunk in &self.base.main_chunks {
            let ch = ch.clone();
            let social = social.clone();
            chunk::send(chunk, move |chunk| {
                game::chunk::update_social(chunk, ch, social, ty);
            });
        }
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

macro_rules! failure {
    ($st: ident, $reason: expr) => {
        match $st {
            SocialState::Friend =>
                FriendAddFailureMessage {
                    reason: $reason
                }.as_packet().unwrap(),
            SocialState::Ignored =>
                IgnoredAddFailureMessage {
                    reason: $reason
                }.as_packet().unwrap(),
        }
    };
}

pub fn add_relation<F>(sender: &Sender, tok: Token, account_id: i32, name: String,
                       st: SocialState, job: F)
                       where F: FnOnce(&mut game::Session, (i32, RelationInformations))
                       + Send + 'static {
    chunk::send(sender, move |server| {
        if let Some(r_id) = server.search_for_player(&name) {
            if r_id == account_id {
                let buf = failure!(st, 3);
                write!(SERVER, tok, buf);
                return;
            }

            let _ = {
                let social = server.session_socials.get_mut(&account_id).unwrap();

                if social.has_relation_with(r_id, st) {
                    let buf = failure!(st, 4);
                    write!(SERVER, tok, buf);
                    return;
                }

                social.add_relation(r_id, st);
            };

            let ch_id = server.session_characters.inv_get(&tok).unwrap();
            server.update_social(
                server.characters.get(ch_id).unwrap(),
                SocialUpdateType::Default
            );

            let infos = {
                let ch_id = server.character_accounts.get(&r_id).unwrap();
                let ch = server.characters.get(ch_id).unwrap();
                ch.as_relation_infos(account_id, server.session_socials.get(&r_id), st)
            };

           server.base.session_callback(tok, move |session, _| job(session, (r_id, infos)));
       } else {
           let buf = failure!(st, 2);
           write!(SERVER, tok, buf);
       }
    });
}


pub fn delete_relation(sender: &Sender, tok: Token, account_id: i32, r_id: i32, st: SocialState) {
    chunk::send(sender, move |server| {
        server.session_socials.get_mut(&account_id).unwrap().remove_relation(r_id, st);
        let ch_id = server.session_characters.inv_get(&tok).unwrap();
        server.update_social(
            server.characters.get(ch_id).unwrap(),
            SocialUpdateType::Default
        );
    })
}

pub fn update_player_status(sender: &Sender, account_id: i32, ch_id: i32,
                            status: PlayerStatusVariant) {
    chunk::send(sender, move |server| {
        server.session_socials.get_mut(&account_id).unwrap().status = status;
        server.update_social(
            server.characters.get(&ch_id).unwrap(),
            SocialUpdateType::Default
        );
    })
}

pub fn update_mood(sender: &Sender, ch_id: i32, mood: i16) {
    chunk::send(sender, move |server| {
        server.characters.get_mut(&ch_id).unwrap().set_mood_smiley(mood);
        server.update_social(server.characters.get(&ch_id).unwrap(), SocialUpdateType::Default);
    })
}
