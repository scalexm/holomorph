use shared::session;
use super::Session;
use shared::chunk;
use std::collections::HashMap;

pub type Chunk = session::chunk::Chunk<Session, ChunkImpl>;
pub type Ref<'a> = session::chunk::Ref<'a, Session, ChunkImpl>;
pub type Sender = chunk::Sender<Chunk>;

pub struct ServerStatus(pub i8, pub String, pub i16);

pub struct ChunkImpl {
    pub game_status: HashMap<i16, ServerStatus>,
}

pub fn new() -> Chunk {
    Chunk::new(ChunkImpl {
        game_status: HashMap::new(),
    })
}

pub fn update_queue(chunk: &Chunk) {
    for session in chunk.sessions.values() {
        session.update_queue();
    }
}

pub fn update_game_server(chunk: &mut Chunk, server_id: i16, status: i8, ip: String,
    port: i16) {

    let _ = chunk.impl_.game_status.insert(server_id, ServerStatus(status, ip, port));
    for session in chunk.sessions.values() {
        session.update_server_status(server_id, status);
    }
}
