pub mod selection;
pub mod identification;

use shared::net::Msg;
use shared::protocol::*;
use shared::protocol::connection::*;
use shared::protocol::enums::server_status;
use shared::protocol::handshake::*;
use shared::protocol::queues::*;
use super::{Session, Chunk};
use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};
use server::data::GameServerData;


static QUEUE_SIZE: AtomicIsize = ATOMIC_ISIZE_INIT;
static QUEUE_COUNTER: AtomicIsize = ATOMIC_ISIZE_INIT;

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
        if self.queue_size == -1 {
            return ();
        }

        let mut pos = self.queue_size -
            (QUEUE_COUNTER.load(Ordering::Relaxed) - self.queue_counter);

        if pos < 0 {
            pos = 0;
        }

        let buf = LoginQueueStatusMessage {
            position: pos as i16,
            total: QUEUE_SIZE.load(Ordering::Relaxed) as i16,
        }.as_packet().unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }

    fn get_server_informations(&self, server: &GameServerData, mut status: i8)
        -> GameServerInformations {

        let data = self.account.as_ref().unwrap();

        if data.is_subscriber() && status == server_status::FULL {
            status = server_status::ONLINE;
        }

        GameServerInformations {
            id: VarShort(server.id),
            status: status,
            completion: 0,
            is_selectable: status == server_status::ONLINE,
            characters_count: *data
                .character_counts
                .get(&server.id)
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

        if server.min_level > self.account.as_ref().unwrap().level {
            return ();
        }

        let buf = ServerStatusUpdateMessage {
            server: self.get_server_informations(&server, status),
        }.as_packet().unwrap();
        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }
}
