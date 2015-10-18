mod approach;
mod character;
mod friend;
mod context;
mod chat;
mod authorized;

use super::{Session, GameState};
use super::chunk::{ChunkImpl, Ref};
use protocol::*;
use protocol::messages::handshake::*;
use protocol::messages::game::approach::*;
use protocol::messages::queues::*;
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
        if let Some(id) = self.account.as_ref().map(|a| a.id) {
            SERVER.with(|s| database::execute(&s.auth_db, move |conn| {
                if let Err(err) = Session::save_auth(id, conn) {
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

    fn save_auth(id: i32, conn: &mut Connection) -> postgres::Result<()> {
        let stmt = try!(conn.prepare_cached("UPDATE accounts SET logged = 0
            WHERE id = $1"));
        let _ = try!(stmt.execute(&[&id]));
        Ok(())
    }

    fn save_game(&self, conn: &mut Connection, ch: Character, map: i32) -> postgres::Result<()> {
        let trans = try!(conn.transaction());
        try!(ch.save(&trans, map));
        trans.commit()
    }
}
