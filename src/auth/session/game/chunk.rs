use server::data::AuthServerData;
use shared::session;
use shared::chunk;
use super::SessionImpl;

pub type Chunk = session::chunk::Chunk<SessionImpl, ChunkImpl>;
pub type Sender = chunk::Sender<Chunk>;

struct ChunkImpl {
    pub server: AuthServerData,
}

pub fn new(server: AuthServerData) -> Chunk {
    Chunk::new(ChunkImpl {
        server: server,
    })
}
