mod approach;
mod character;
mod friend;
mod context;

use super::{Session, SessionImpl, QueueState};
use super::chunk::Chunk;
use shared::session;
use shared::protocol::*;
use shared::protocol::messages::handshake::*;
use shared::protocol::messages::game::approach::*;
use shared::protocol::messages::queues::*;
use std::io::{self, Cursor};
use shared::net::{Token, Msg};
use std::sync::atomic::Ordering;
use shared::database;
use postgres::{self, Connection};
use std::collections::HashMap;

fn save_auth(self_: &Session, conn: &mut Connection) -> postgres::Result<()> {
    let account = match self_.account.as_ref() {
        Some(account) => account,
        None => return Ok(()),
    };

    let stmt = try!(conn.prepare_cached("UPDATE accounts SET logged = 0
        WHERE id = $1"));
    try!(stmt.execute(&[&account.id]));

    Ok(())
}

impl session::SessionImpl for SessionImpl {
    type Chunk = Chunk;

    fn new(token: Token, chunk: &Chunk) -> Self {
        let mut buf = ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet().unwrap();

        HelloGameMessage.as_packet_with_buf(&mut buf).unwrap();

        send!(chunk, Msg::Write(token, buf));

        SessionImpl {
            queue_state: QueueState::None,
            account: None,
            characters: HashMap::new(),
            current_character: None,
        }
    }

    fn get_handler(id: u16)
        -> (fn(&mut Session, &Chunk, Cursor<Vec<u8>>) -> io::Result<()>) {

        use self::{approach, character, friend, context};

        match id {
            110 => approach::handle_authentication_ticket,
            150 => character::handle_characters_list_request,
            152 => character::handle_character_selection,
            250 => context::handle_game_context_create_request,
            225 => context::handle_map_informations_request,
            4001 => friend::handle_friends_get_list,
            5676 => friend::handle_ignored_get_list,
            950 => context::handle_game_map_movement_request,
            221 => context::handle_change_map,
            _ => SessionImpl::unhandled,
        }
    }

    fn close(self_: Session, chunk: &Chunk) {
        database::execute(&chunk.server.auth_db, move |conn| {
            if let Err(err) = save_auth(&self_, conn) {
                error!("error while saving session to auth db: {:?}", err);
            }
        });
    }
}

pub fn update_queue(self_: &Session, chunk: &Chunk) {
    let (QUEUE_SIZE, QUEUE_COUNTER) = match self_.queue_state {
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

    let (queue_size, queue_counter) = match self_.queue_state {
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

    send!(chunk, Msg::Write(self_.token, buf));
}
