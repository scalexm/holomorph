use shared::pool;
use server;
use shared::net;
use shared::database;
use config::Config;
use std::sync::Arc;
use std::collections::HashMap;
use postgres::Result;

pub struct GameServerData {
    pub id: i16,
    pub key: String,
    pub min_level: i8,
}

#[derive(Clone)]
pub struct AuthServerData {
    pub handler: server::Sender,
    pub io_loop: net::Sender,
    pub db: database::Sender,
    pub key: Arc<Vec<u8>>,
    pub patch: Arc<Vec<u8>>,
    pub cnf: Arc<Config>,
    pub game_servers: Arc<HashMap<i16, GameServerData>>,
}


impl AuthServerData {
    pub fn new(handler: server::Sender, io_loop: net::Sender,
        db: database::Sender, key: Vec<u8>, patch: Vec<u8>,
        cnf: Config) -> AuthServerData {

            AuthServerData {
                handler: handler,
                io_loop: io_loop,
                db: db,
                key: Arc::new(key),
                patch: Arc::new(patch),
                cnf: Arc::new(cnf),
                game_servers: Arc::new(HashMap::new()),
            }
    }

    pub fn load(&mut self) -> Result<()> {
        let conn = database::connect(&self.cnf.database_uri);

        let stmt = try!(conn.prepare("SELECT * FROM game_servers"));
        let mut game_servers = HashMap::new();
        for row in &try!(stmt.query(&[])) {
            let id: i16 = row.get("id");
            let min_level: i16 = row.get("min_level");
            let _ = game_servers.insert(id, GameServerData {
                id: id,
                key: row.get("key"),
                min_level: min_level as i8,
            });
        }
        self.game_servers = Arc::new(game_servers);

        Ok(())
    }

    pub fn shutdown(&self) {
        let _ = self.io_loop.send(net::Msg::Shutdown);
        let _ = self.handler.send(pool::Msg::Shutdown);
    }
}
