pub mod selection;
pub mod identification;

use shared::net::{Token, Msg};
use shared::protocol::*;
use shared::protocol::messages::connection::*;
use shared::protocol::enums::server_status;
use shared::protocol::messages::handshake::*;
use shared::protocol::messages::queues::*;
use shared::protocol::types::connection::GameServerInformations;
use super::{Session, SessionImpl, QueueState};
use std::sync::atomic::Ordering;
use server::data::GameServerData;
use shared::session;
use super::chunk::Chunk;
use std::io::{self, Cursor};

impl session::SessionImpl for SessionImpl {
    type Chunk = Chunk;

    fn new(token: Token, chunk: &Chunk) -> Self {
        let mut buf = Vec::new();

        ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet_with_buf(&mut buf).unwrap();

        HelloConnectMessage {
            salt: "salut".to_string(),
            key: VarIntVec((*chunk.server.key).clone()),
        }.as_packet_with_buf(&mut buf).unwrap();

        send!(chunk, Msg::Write(token, buf));

        SessionImpl {
            account: None,
            queue_state: QueueState::None,
            custom_identification: false,
            aes_key: Vec::new(),
        }
    }

    fn get_handler(id: u16)
        -> (fn(&mut Session, &Chunk, Cursor<Vec<u8>>) -> io::Result<()>) {

        use self::{selection, identification};

        match id {
            4 => identification::handle_identification,
            40 => selection::handle_server_selection,
            _ => SessionImpl::unhandled,
        }
    }

    fn close(_: Session, _: &Chunk) { }
}

pub fn update_queue(self_: &Session, chunk: &Chunk) {
    if let QueueState::Some(queue_size, queue_counter) = self_.queue_state {
        use self::identification::{QUEUE_COUNTER, QUEUE_SIZE};

        let mut pos = queue_size -
            (QUEUE_COUNTER.load(Ordering::Relaxed) - queue_counter);

        if pos < 0 {
            pos = 0;
        }

        let buf = LoginQueueStatusMessage {
            position: pos as i16,
            total: QUEUE_SIZE.load(Ordering::Relaxed) as i16,
        }.as_packet().unwrap();

        send!(chunk, Msg::Write(self_.token, buf));
    }
}

fn get_server_informations(self_: &Session, server: &GameServerData, mut status: i8)
    -> GameServerInformations {

    let data = self_.account.as_ref().unwrap();

    if data.is_subscriber() && status == server_status::FULL {
        status = server_status::ONLINE;
    }

    GameServerInformations {
        id: VarShort(server.id()),
        status: status,
        completion: 0,
        is_selectable: status == server_status::ONLINE,
        characters_count: *data
            .character_counts
            .get(&server.id())
            .unwrap_or(&0),
        date: 0.,
    }
}

pub fn update_server_status(self_: &Session, chunk: &Chunk, server_id: i16, status: i8) {
    let account = match self_.account.as_ref() {
        Some(account) => account,
        None => return (),
    };

    let server = match chunk.server.game_servers.get(&server_id) {
        Some(server) => server,
        None => return (),
    };

    if server.min_level() > account.level {
        return ();
    }

    let buf = ServerStatusUpdateMessage {
        server: get_server_informations(self_, server, status),
    }.as_packet().unwrap();

    send!(chunk, Msg::Write(self_.token, buf));
}
