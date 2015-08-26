pub mod approach;
pub mod characters_list;

use super::{Session, Chunk, QueueState};
use shared::protocol::*;
use shared::protocol::messages::handshake::*;
use shared::protocol::messages::game::approach::*;
use shared::protocol::messages::queues::*;
use std::io::{self, Cursor};
use shared::net::Msg;
use std::sync::atomic::Ordering;

impl Session {
    pub fn start(&self, chunk: &Chunk) {
        let mut buf = Vec::new();

        ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet_with_buf(&mut buf).unwrap();

        HelloGameMessage.as_packet_with_buf(&mut buf).unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }

    pub fn update_queue(&self, chunk: &Chunk) {
        let (QUEUE_SIZE, QUEUE_COUNTER) = match self.queue_state {
            QueueState::SomeTicket(..) => {
                use self::approach::{QUEUE_COUNTER, QUEUE_SIZE};
                (QUEUE_COUNTER.load(Ordering::Relaxed),
                    QUEUE_SIZE.load(Ordering::Relaxed))
            }

            _ => return (),
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

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }
}
