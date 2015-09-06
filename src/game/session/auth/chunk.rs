use shared::chunk;
use shared::session;
use super::SessionImpl;
use server::data::GameServerData;

pub type Chunk = session::chunk::Chunk<SessionImpl, ChunkImpl>;
pub type Sender = chunk::Sender<Chunk>;

struct ChunkImpl {
    pub connected: bool,
    pub server: GameServerData,
}

pub fn new(server: GameServerData) -> Chunk {
    Chunk::new(ChunkImpl {
        connected: false,
        server: server,
    })
}
