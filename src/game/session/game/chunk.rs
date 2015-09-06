use server::data::GameServerData;
use shared::chunk;
use shared::session;
use super::{SessionImpl, handlers};

pub type Chunk = session::chunk::Chunk<SessionImpl, ChunkImpl>;
pub type Sender = chunk::Sender<Chunk>;

struct ChunkImpl {
    pub server: GameServerData,
}

pub fn new(server: GameServerData) -> Chunk {
    Chunk::new(ChunkImpl {
        server: server,
    })
}

pub fn update_queue(self_: &Chunk) {
    for session in self_.sessions.values() {
        handlers::update_queue(&session.borrow(), self_);
    }
}
