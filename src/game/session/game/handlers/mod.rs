macro_rules! get_mut_character {
    ($ch: ident, $chunk: ident) => {
        $chunk.maps
            .get_mut(&$ch.map_id).unwrap()
            .get_mut_actor($ch.id).unwrap()
            .as_mut_character()
    };
}

macro_rules! get_character {
    ($ch: ident, $chunk: ident) => {
        $chunk.maps
            .get(&$ch.map_id).unwrap()
            .get_actor($ch.id).unwrap()
            .as_character()
    };
}

mod approach;
mod character;
mod friend;
mod context;

use super::{Session, GameState};
use super::chunk::{ChunkImpl, Ref};
use shared::session::{self, SessionBase};
use shared::protocol::*;
use shared::protocol::messages::handshake::*;
use shared::protocol::messages::game::approach::*;
use shared::protocol::messages::queues::*;
use std::io::{self, Cursor};
use std::sync::atomic::Ordering;
use shared::database;
use postgres::{self, Connection};
use server::SERVER;
use character::Character;
use std::mem;

impl session::Session<ChunkImpl> for Session {
    fn new(base: SessionBase) -> Self {
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
        }
    }

    fn get_handler<'a>(id: u16)
        -> (fn(&mut Session, Ref<'a>, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            110 => Session::handle_authentication_ticket,
            150 => Session::handle_characters_list_request,
            152 => Session::handle_character_selection,
            250 => Session::handle_game_context_create_request,
            225 => Session::handle_map_informations_request,
            4001 => Session::handle_friends_get_list,
            5676 => Session::handle_ignored_get_list,
            950 => Session::handle_game_map_movement_request,
            952 => Session::handle_game_map_movement_confirm,
            953 => Session::handle_game_map_movement_cancel,
            221 => Session::handle_change_map,
            _ => Session::unhandled,
        }
    }

    fn close<'a>(mut self, mut chunk: Ref<'a>) {
        if let Some(account) = self.account.as_ref() {
            let id = account.id;

            SERVER.with(|s| database::execute(&s.auth_db, move |conn| {
                if let Err(err) = Session::save_auth(id, conn) {
                    error!("error while saving session to auth db: {:?}", err);
                }
            }));
        }

        let state = mem::replace(&mut self.state, GameState::None);
        if let GameState::InContext(ch) = state {
            let map_id = ch.map_id;
            let ch = chunk.maps.get_mut(&ch.map_id).unwrap()
                .remove_actor(ch.id).unwrap()
                .into_character();

            SERVER.with(|s| database::execute(&s.db, move |conn| {
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
                (QUEUE_COUNTER.load(Ordering::Relaxed),
                    QUEUE_SIZE.load(Ordering::Relaxed))
            }

            GameState::GameQueue(..) => {
                use self::character::{QUEUE_COUNTER, QUEUE_SIZE};
                (QUEUE_COUNTER.load(Ordering::Relaxed),
                    QUEUE_SIZE.load(Ordering::Relaxed))
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

    fn save_game(&self, conn: &mut Connection, ch: Character, map: i32)
        -> postgres::Result<()> {

        let trans = try!(conn.transaction());
        try!(ch.save(&trans, map));
        trans.commit()
    }
}
