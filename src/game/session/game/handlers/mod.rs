mod approach;
mod character;
mod friend;
mod context;

use super::{Session, QueueState};
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
use std::collections::HashMap;
use server::SERVER;

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
            queue_state: QueueState::None,
            account: None,
            characters: HashMap::new(),
            current_character: None,
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
            221 => Session::handle_change_map,
            _ => Session::unhandled,
        }
    }

    fn close<'a>(self, _: Ref<'a>) {
        SERVER.with(|s| database::execute(&s.auth_db, move |conn| {
            if let Err(err) = self.save_auth(conn) {
                error!("error while saving session to auth db: {:?}", err);
            }
        }));
    }
}

impl Session {
    pub fn update_queue(&self) {
        let (QUEUE_SIZE, QUEUE_COUNTER) = match self.queue_state {
            QueueState::SomeTicket(..) => {
                use self::approach::{QUEUE_COUNTER, QUEUE_SIZE};
                (QUEUE_COUNTER.load(Ordering::Relaxed),
                    QUEUE_SIZE.load(Ordering::Relaxed))
            }

            QueueState::SomeGame(..) => {
                use self::character::{QUEUE_COUNTER, QUEUE_SIZE};
                (QUEUE_COUNTER.load(Ordering::Relaxed),
                    QUEUE_SIZE.load(Ordering::Relaxed))
            }

            QueueState::None => return (),
        };

        let (queue_size, queue_counter) = match self.queue_state {
            QueueState::SomeTicket(qs, qc) | QueueState::SomeGame(qs, qc) => (qs, qc),
            QueueState::None => unreachable!(),
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

    fn save_auth(&self, conn: &mut Connection) -> postgres::Result<()> {
        let account = match self.account.as_ref() {
            Some(account) => account,
            None => return Ok(()),
        };

        let stmt = try!(conn.prepare_cached("UPDATE accounts SET logged = 0
            WHERE id = $1"));
        try!(stmt.execute(&[&account.id]));

        Ok(())
    }
}
