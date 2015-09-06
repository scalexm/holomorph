use shared::session;
use server::data::AuthServerData;
use super::{handlers, SessionImpl};
use shared::chunk;
use std::collections::HashMap;

pub type Chunk = session::chunk::Chunk<SessionImpl, ChunkImpl>;
pub type Sender = chunk::Sender<Chunk>;

struct ServerStatus(pub i8, pub String, pub i16);

struct ChunkImpl {
    pub server: AuthServerData,
    pub game_status: HashMap<i16, ServerStatus>,
}

pub fn new(server: AuthServerData) -> Chunk {
    Chunk::new(ChunkImpl {
        server: server,
        game_status: HashMap::new(),
    })
}

pub fn update_queue(self_: &Chunk) {
    for session in self_.sessions.values() {
        handlers::update_queue(&session.borrow(), self_);
    }
}

pub fn update_game_server(self_: &mut Chunk, server_id: i16, status: i8, ip: String,
    port: i16) {

    let _ = self_.game_status.insert(server_id, ServerStatus(status, ip, port));
    for session in self_.sessions.values() {
        handlers::update_server_status(&session.borrow(), self_, server_id, status);
    }
}
