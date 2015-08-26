use server;
use shared::{net, pool, database};
use config::Config;
use std::sync::Arc;
use std::collections::HashMap;
use postgres::Result;
use postgres::rows::Row;

pub struct GameServerData {
    id: i16,
    key: String,
    min_level: i8,
}

impl GameServerData {
    pub fn from_sql<'a>(row: Row<'a>) -> (i16, GameServerData) {
        let id = row.get("id");
        assert!(id > 0); // id 0 is used as a null value
        let min_level: i16 = row.get("min_level");

        (id, GameServerData {
            id: id,
            key: row.get("key"),
            min_level: min_level as i8,
        })
    }

    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn min_level(&self) -> i8 {
        self.min_level
    }
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
        self.game_servers = Arc::new(try!(stmt.query(&[])).iter().map(|row|
            GameServerData::from_sql(row)).collect());
        info!("loaded {} game servers", self.game_servers.len());

        Ok(())
    }

    pub fn shutdown(&self) {
        let _ = self.io_loop.send(net::Msg::Shutdown);
        let _ = self.handler.send(pool::Msg::Shutdown);
    }
}
