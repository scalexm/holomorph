mod approach;
mod character;
mod friend;
mod context;
mod chat;
mod authorized;

use super::{Session, GameState, AccountData};
use super::chunk::{ChunkImpl, Ref, SocialState};
use character::CharacterMinimal;
use protocol::*;
use protocol::messages::handshake::*;
use protocol::messages::game::approach::*;
use protocol::messages::queues::*;
use protocol::messages::game::basic::TextInformationMessage;
use protocol::enums::text_information_type;
use std::io::{Result, Cursor};
use std::sync::atomic::Ordering;
use shared::{self, database};
use postgres::{self, Connection};
use server::SERVER;
use character::Character;
use std::mem;
use std::collections::HashMap;

impl shared::session::Session<ChunkImpl> for Session {
    fn new(base: shared::session::SessionBase) -> Self {
        let mut buf = ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet().unwrap();

        HelloGameMessage.as_packet_with_buf(&mut buf).unwrap();

        write!(SERVER, base.token, buf);

        Session {
            base: base,
            account: None,
            state: GameState::None,

            last_sales_chat_request: 0,
            last_seek_chat_request: 0,

            friends: HashMap::new(),
            ignored: HashMap::new(),
        }
    }

    fn get_handler<'a>(id: u16) -> (fn(&mut Session, Ref<'a>, Cursor<Vec<u8>>) -> Result<()>) {
        match id {
            110 => Session::handle_authentication_ticket,

            150 => Session::handle_characters_list_request,
            152 => Session::handle_character_selection,

            4001 => Session::handle_friends_get_list,
            5676 => Session::handle_ignored_get_list,
            5602 => Session::handle_friend_set_warn_on_connection,
            6077 => Session::handle_friend_set_warn_on_level_gain,

            250 => Session::handle_game_context_create_request,
            225 => Session::handle_map_informations_request,

            950 => Session::handle_game_map_movement_request,
            952 => Session::handle_game_map_movement_confirm,
            953 => Session::handle_game_map_movement_cancel,
            221 => Session::handle_change_map,

            861 => Session::handle_chat_client_multi,
            862 => Session::handle_chat_client_multi_with_object,

            5662 => Session::handle_admin_quiet_command_message,

            _ => Session::unhandled,
        }
    }

    fn close<'a>(mut self, mut chunk: Ref<'a>) {
        let account = mem::replace(&mut self.account, None);
        if let Some(account) = account {
            SERVER.with(|s| database::execute(&s.auth_db, move |conn| {
                if let Err(err) = Session::save_auth(account, conn) {
                    error!("error while saving session to auth db: {:?}", err);
                }
            }));
        }

        let state = mem::replace(&mut self.state, GameState::None);
        if let GameState::InContext(ch) = state {
            let map_id = ch.map_id;
            let ch = chunk.maps
                          .get_mut(&ch.map_id).unwrap()
                          .remove_actor(ch.id).unwrap()
                          .into_character();

            SERVER.with(|s| database::execute(&s.db, move |conn| {
                if let Err(err) = self.base.save_logs(conn, ch.minimal().account_id()) {
                    error!("error while saving logs: {:?}", err);
                }

                if let Err(err) = self.save_game(conn, ch, map_id) {
                    error!("error while saving session to game db: {:?}", err);
                }
            }));
        }
    }
}

impl Session {
    pub fn update_queue(&self) {
        let (QUEUE_SIZE, QUEUE_COUNTER) = match self.state {
            GameState::TicketQueue(..) => {
                use self::approach::{QUEUE_COUNTER, QUEUE_SIZE};
                (QUEUE_COUNTER.load(Ordering::Relaxed), QUEUE_SIZE.load(Ordering::Relaxed))
            }

            GameState::GameQueue(..) => {
                use self::character::{QUEUE_COUNTER, QUEUE_SIZE};
                (QUEUE_COUNTER.load(Ordering::Relaxed), QUEUE_SIZE.load(Ordering::Relaxed))
            }

            _ => return (),
        };

        let (queue_size, queue_counter) = match self.state {
            GameState::TicketQueue(qs, qc) | GameState::GameQueue(qs, qc) => (qs, qc),
            _ => unreachable!(),
        };

        let mut pos = queue_size - (QUEUE_COUNTER - queue_counter);

        if pos < 0 {
            pos = 0;
        }

        let buf = QueueStatusMessage {
            position: pos as i16,
            total: QUEUE_SIZE as i16,
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);
    }

    pub fn update_social(&mut self, ch: &CharacterMinimal, state: SocialState) {
        let account = match self.account.as_ref() {
            Some(account) => account,
            None => return,
        };

        let account_id = ch.account_id();

        if account.social.friends.contains(&account_id) {
            let infos = ch.as_friend_infos(account.id, state);
            let _ = self.friends.insert(account_id, infos);

            match state {
                SocialState::Online if account.social.warn_on_connection
                                       && ch.is_friend_with(account.id) => {
                    let buf = TextInformationMessage {
                        msg_type: text_information_type::MESSAGE,
                        msg_id: VarShort(143),
                        parameters: vec![ch.name().to_string(), ch.account_nickname().to_string(),
                                         account_id.to_string()],
                    }.as_packet().unwrap();
                    write!(SERVER, self.base.token, buf);
                },

                SocialState::UpdateWithLevel(lvl) if account.social.warn_on_level_gain => {
                    // TODO
                },

                _ => (),
            }
        }

        if account.social.ignored.contains(&account_id) {
            let infos = ch.as_ignored_infos(state);
            let _ = self.ignored.insert(account_id, infos);
        }
    }

    fn save_auth(account: AccountData, conn: &mut Connection) -> postgres::Result<()> {
        let stmt = try!(conn.prepare_cached("UPDATE accounts SET logged = 0
            WHERE id = $1"));
        let _ = try!(stmt.execute(&[&account.id]));

        let stmt = try!(conn.prepare_cached("UPDATE friends SET friends = $1,
            ignored = $2, warn_on_connection = $3, warn_on_level_gain = $4
            WHERE account_id = $5"));
        let friends = account.social.friends.iter()
                                            .map(|&id| id.to_string())
                                            .collect::<Vec<_>>();
        let friends = friends.join(",");
        let ignored = account.social.ignored.iter()
                                             .map(|&id| id.to_string())
                                             .collect::<Vec<_>>();
        let ignored = ignored.join(",");
        let _ = try!(stmt.execute(&[&friends, &ignored, &account.social.warn_on_connection,
                                    &account.social.warn_on_level_gain, &account.id]));

        Ok(())
    }

    fn save_game(&self, conn: &mut Connection, ch: Character, map: i32) -> postgres::Result<()> {
        let trans = try!(conn.transaction());
        try!(ch.save(&trans, map));
        trans.commit()
    }
}
