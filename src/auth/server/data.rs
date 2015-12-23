use shared::{net, database};
use config::Config;
use std::sync::Arc;
use std::collections::HashMap;
use diesel::*;
use server;

#[derive(Queriable)]
pub struct GameServerData {
    id: i16,
    key: String,
    min_level: i16,
}

impl GameServerData {
    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn min_level(&self) -> i16 {
        self.min_level
    }
}

#[derive(Clone)]
pub struct AuthServerData {
    pub server: server::Sender,
    pub io_loop: net::Sender,
    pub db: database::Sender,
    pub key: Arc<Vec<u8>>,
    pub patch: Arc<Vec<u8>>,
    pub cnf: Arc<Config>,
    pub game_servers: Arc<HashMap<i16, GameServerData>>,
}

impl AuthServerData {
    pub fn new(server: server::Sender, io_loop: net::Sender, db: database::Sender, key: Vec<u8>,
               patch: Vec<u8>, cnf: Config) -> Self {
            AuthServerData {
                server: server,
                io_loop: io_loop,
                db: db,
                key: Arc::new(key),
                patch: Arc::new(patch),
                cnf: Arc::new(cnf),
                game_servers: Arc::new(HashMap::new()),
            }
    }

    pub fn load(&mut self, conn: &Connection) {
        use shared::database::schema::game_servers;

        self.game_servers = Arc::new(
            game_servers::table.load::<GameServerData>(conn)
                               .unwrap()
                               .map(|gs| (gs.id(), gs))
                               .collect()
        );
        info!("loaded {} game servers", self.game_servers.len());
    }
}
