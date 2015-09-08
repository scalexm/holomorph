use shared::chunk;
use shared::session;
use super::Session;

pub type Chunk = session::chunk::Chunk<Session, ChunkImpl>;
pub type Ref<'a> = session::chunk::Ref<'a, Session, ChunkImpl>;
pub type Sender = chunk::Sender<Chunk>;

pub struct ChunkImpl;

pub fn new() -> Chunk {
    Chunk::new(ChunkImpl)
}

pub fn update_queue(chunk: &Chunk) {
    for session in chunk.sessions.values() {
        session.update_queue();
    }
}
