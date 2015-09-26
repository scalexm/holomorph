pub mod selection;
pub mod identification;

use shared::protocol::*;
use shared::protocol::messages::connection::*;
use shared::protocol::messages::handshake::*;
use shared::protocol::messages::queues::*;
use shared::protocol::types::connection::GameServerInformations;
use super::{Session, QueueState};
use std::sync::atomic::Ordering;
use server::data::GameServerData;
use shared::session::{self, SessionBase};
use super::chunk::{ChunkImpl, Ref};
use std::io::{self, Cursor};
use server::SERVER;
use shared::protocol::enums::server_status;

impl session::Session<ChunkImpl> for Session {
    fn new(base: SessionBase) -> Self {
        let mut buf = Vec::new();

        ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet_with_buf(&mut buf).unwrap();

        HelloConnectMessage {
            salt: "salut".to_string(),
            key: VarIntVec(SERVER.with(|s| (*s.key).clone())),
        }.as_packet_with_buf(&mut buf).unwrap();

        write!(SERVER, base.token, buf);

        Session {
            base: base,
            account: None,
            queue_state: QueueState::None,
            custom_identification: false,
            aes_key: Vec::new(),
        }
    }

    fn get_handler<'a>(id: u16)
        -> (fn(&mut Session, Ref<'a>, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            4 => Session::handle_identification,
            40 => Session::handle_server_selection,
            _ => Session::unhandled,
        }
    }

    fn close<'a>(self, _: Ref<'a>) { }
}

impl Session {
    pub fn update_queue(&self) {
        if let QueueState::Some(queue_size, queue_counter) = self.queue_state {
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

            write!(SERVER, self.base.token, buf);
        }
    }

    fn get_server_informations(&self, server: &GameServerData, mut status: i8)
        -> GameServerInformations {

        let data = self.account.as_ref().unwrap();

        if data.is_subscriber() && status == server_status::FULL {
            status = server_status::ONLINE;
        }

        GameServerInformations {
            id: VarShort(server.id()),
            type_: 0,
            status: status,
            completion: 0,
            is_selectable: status == server_status::ONLINE,
            characters_count: *data
                .character_counts
                .get(&server.id())
                .unwrap_or(&0),
            characters_slots: 0,
            date: 0.,
        }
    }

    pub fn update_server_status(&self, server_id: i16, status: i8) {
        let account = match self.account.as_ref() {
            Some(account) => account,
            None => return (),
        };

        SERVER.with(|s| {
            let server = s.game_servers.get(&server_id).unwrap();

            if server.min_level() > account.level {
                return ();
            }

            let buf = ServerStatusUpdateMessage {
                server: self.get_server_informations(server, status),
            }.as_packet().unwrap();

            write!(SERVER, self.base.token, buf);
        });
    }
}
