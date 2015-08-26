pub mod selection;
pub mod identification;

use shared::net::Msg;
use shared::protocol::*;
use shared::protocol::messages::connection::*;
use shared::protocol::enums::server_status;
use shared::protocol::messages::handshake::*;
use shared::protocol::messages::queues::*;
use shared::protocol::types::connection::GameServerInformations;
use super::{Session, Chunk, QueueState};
use std::sync::atomic::Ordering;
use server::data::GameServerData;

impl Session {
    pub fn start(&self, chunk: &Chunk) {
        let mut buf = Vec::new();

        ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet_with_buf(&mut buf).unwrap();

        HelloConnectMessage {
            salt: "salut".to_string(),
            key: VarIntVec((*chunk.server.key).clone()),
        }.as_packet_with_buf(&mut buf).unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }

    pub fn update_queue(&self, chunk: &Chunk) {
        use self::identification::{QUEUE_COUNTER, QUEUE_SIZE};

        if let QueueState::Some(queue_size, queue_counter) = self.queue_state {
            let mut pos = queue_size -
                (QUEUE_COUNTER.load(Ordering::Relaxed) - queue_counter);

            if pos < 0 {
                pos = 0;
            }

            let buf = LoginQueueStatusMessage {
                position: pos as i16,
                total: QUEUE_SIZE.load(Ordering::Relaxed) as i16,
            }.as_packet().unwrap();

            let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
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

    pub fn update_server_status(&self, chunk: &Chunk, server_id: i16, status: i8) {
        if self.account.is_none() {
            return ();
        }

        let server = chunk.server.game_servers.get(&server_id);
        if server.is_none() {
            return ();
        }

        let server = server.unwrap();

        if server.min_level() > self.account.as_ref().unwrap().level {
            return ();
        }

        let buf = ServerStatusUpdateMessage {
            server: self.get_server_informations(&server, status),
        }.as_packet().unwrap();
        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }
}
