use shared;
use super::Session;

pub type Chunk = shared::session::chunk::Chunk<Session, ChunkImpl>;
pub type Ref<'a> = shared::session::chunk::Ref<'a, Session, ChunkImpl>;
pub type Sender = shared::chunk::Sender<Chunk>;

pub struct ChunkImpl;

pub fn new() -> Chunk {
    Chunk::new(ChunkImpl)
}
