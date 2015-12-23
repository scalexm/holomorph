use super::Server;
use shared::net::Token;
use shared::chunk;
use protocol::types::game::character::status::PlayerStatus;
use protocol::variants::PlayerStatusVariant;
use protocol::*;
use protocol::messages::game::friend::*;
use session::game::chunk::SocialUpdateType;
use session::game::{self, Session, SocialState};
use protocol::types::game::data::items::ObjectItem;
use character::CharacterMinimal;
use character::social::RelationInformations;
use super::{Sender, SERVER};
use protocol::messages::game::chat::{
    ChatErrorMessage,
    ChatAbstractServerMessage,
    ChatServerMessage,
    ChatServerCopyMessage,
    ChatServerWithObjectMessage,
    ChatServerCopyWithObjectMessage
};
use protocol::messages::game::basic::TextInformationMessage;
use protocol::enums::{text_information_type, chat_error, player_status};
use time;

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

macro_rules! build_message {
    ($msg: ident, $items: expr) => {
        if $items.len() > 0 {
            ChatServerWithObjectMessage {
                base: $msg,
                objects: $items,
            }.as_packet().unwrap()
        }
        else {
            $msg.as_packet().unwrap()
        }
    };
}

macro_rules! build_copy_message {
    ($msg: ident, $items: expr, $buf: ident) => {
        if $items.len() > 0 {
            ChatServerCopyWithObjectMessage {
                base: $msg,
                objects: $items,
            }.as_packet_with_buf(&mut $buf).unwrap()
        }
        else {
            $msg.as_packet_with_buf(&mut $buf).unwrap()
        }
    };
}

pub fn send_private_message(sender: &Sender, tok: Token, sender_id: i32, sender_name: String,
                            sender_ch_id: i32, receiver: String, content: String,
                            items: Vec<ObjectItem>) {
    chunk::send(sender, move |server| {
        if let Some(r_id) = server.search_for_player(&receiver) {
            if r_id == sender_id {
                let buf = ChatErrorMessage {
                    reason: chat_error::INTERIOR_MONOLOGUE,
                }.as_packet().unwrap();
                write!(SERVER, tok, buf);
                return;
            }

            if let Some(&r_tok) = server.session_accounts.get(&r_id) {
                let r_social = server.session_socials.get(&r_id).unwrap();
                let &r_ch_id = server.session_characters.inv_get(&r_tok).unwrap();
                let r_name = server.characters.get(&r_ch_id)
                                              .unwrap()
                                              .name();

                if r_social.has_relation_with(sender_id, SocialState::Ignored) {
                    let buf = TextInformationMessage {
                        msg_type: text_information_type::ERROR,
                        msg_id: VarShort(381),
                        parameters: vec![r_name.to_string()],
                    }.as_packet().unwrap();
                    write!(SERVER, tok, buf);
                    return;
                }

                if r_social.status.status_id() == player_status::SOLO {
                    let buf = TextInformationMessage {
                        msg_type: text_information_type::ERROR,
                        msg_id: VarShort(367),
                        parameters: vec![r_name.to_string()],
                    }.as_packet().unwrap();
                    write!(SERVER, tok, buf);
                    return;
                }

                if r_social.status.status_id() == player_status::PRIVATE
                    && !r_social.has_relation_with(sender_id, SocialState::Friend) {

                    let buf = TextInformationMessage {
                        msg_type: text_information_type::ERROR,
                        msg_id: VarShort(366),
                        parameters: vec![r_name.to_string()],
                    }.as_packet().unwrap();
                    write!(SERVER, tok, buf);
                    return;
                }

                // switch player status to available if needed
                let s_social = server.session_socials.get(&sender_id).unwrap();
                let status_id = s_social.status.status_id();
                if status_id == player_status::SOLO || status_id == player_status::AFK
                    || (status_id == player_status::PRIVATE
                        && !s_social.has_relation_with(r_id, SocialState::Friend)) {

                    server.base.session_callback(tok, |session, _| {
                        session.update_status(PlayerStatusVariant::PlayerStatus(PlayerStatus {
                            status_id: player_status::AVAILABLE,
                        }))
                    });
                }

                let msg_abstract = ChatAbstractServerMessage {
                    channel: 9,
                    content: content,
                    timestamp: time::get_time().sec as i32,
                    fingerprint: String::new(),
                };

                let msg = ChatServerMessage {
                    base: msg_abstract.clone(),
                    sender_id: sender_ch_id,
                    sender_name: sender_name,
                    sender_account_id: sender_id,
                };

                let msg_copy = ChatServerCopyMessage {
                    base: msg_abstract,
                    receiver_id: VarInt(r_ch_id),
                    receiver_name: r_name.to_string(),
                };

                let r_buf = build_message!(msg, items.clone());
                write!(SERVER, r_tok, r_buf);

                let mut s_buf = Vec::new();
                build_copy_message!(msg_copy, items, s_buf);

                if r_social.status.status_id() == player_status::AFK {
                    let (msg_id, params) = match r_social.status {
                        PlayerStatusVariant::PlayerStatusExtended(ref status) =>
                            (364, vec![r_name.to_string(), status.message.clone()]),
                        _ => (369, vec![r_name.to_string()]),
                    };

                    TextInformationMessage {
                        msg_type: text_information_type::ERROR,
                        msg_id: VarShort(msg_id),
                        parameters: params,
                    }.as_packet_with_buf(&mut s_buf).unwrap();
                }

                write!(SERVER, tok, s_buf);
                return;
            }
        }

        let buf = ChatErrorMessage {
            reason: chat_error::RECEIVER_NOT_FOUND,
        }.as_packet().unwrap();
        write!(SERVER, tok, buf);
    })
}
