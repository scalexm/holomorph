mod map;

use server;
use std::sync::Arc;
use config::Config;
use shared::{net, chunk};
use shared::database;
use postgres::Result;
use character::CharacterMinimal;
use server::Handler;
use self::map::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct GameServerData {
    pub handler: server::Sender,
    pub io_loop: net::Sender,
    pub cnf: Arc<Config>,
    pub auth_db: database::Sender,
    pub db: database::Sender,
    pub maps: Arc<HashMap<i32, MapData>>,
    pub sub_areas: Arc<HashMap<i16, SubAreaData>>,
    pub areas: Arc<HashMap<i16, AreaData>>,
}

impl GameServerData {
    pub fn new(handler: server::Sender, io_loop: net::Sender,
        cnf: Config, db: database::Sender, auth_db: database::Sender) -> Self {

        GameServerData {
            handler: handler,
            io_loop: io_loop,
            cnf: Arc::new(cnf),
            auth_db: auth_db,
            db: db,
            maps: Arc::new(HashMap::new()),
            sub_areas: Arc::new(HashMap::new()),
            areas: Arc::new(HashMap::new()),
        }
    }

    pub fn load(&mut self) -> Result<()> {
        let conn = database::connect(&self.cnf.database_uri);

        let stmt = try!(conn.prepare("SELECT * FROM map_positions JOIN maps
            ON map_positions.id = maps.id"));
        self.maps = Arc::new(try!(stmt.query(&[])).iter().map(|row|
            MapData::from_sql(row)).collect());
        info!("loaded {} maps", self.maps.len());

        let stmt = try!(conn.prepare("SELECT * FROM sub_areas"));
        self.sub_areas = Arc::new(try!(stmt.query(&[])).iter().map(|row|
            SubAreaData::from_sql(row)).collect());
        info!("loaded {} sub areas", self.sub_areas.len());

        let stmt = try!(conn.prepare("SELECT * FROM areas"));
        self.areas = Arc::new(try!(stmt.query(&[])).iter().map(|row|
            AreaData::from_sql(row)).collect());
        info!("loaded {} areas", self.areas.len());

        Ok(())
    }

    pub fn shutdown(&self) {
        let _ = self.io_loop.send(net::Msg::Shutdown);
        let _ = self.handler.send(chunk::Msg::Shutdown);
    }
}

impl Handler {
    pub fn load(&mut self, uri: &str) -> Result<()> {
        let conn = database::connect(uri);

        let stmt = try!(conn.prepare("SELECT * FROM character_minimals"));
        self.characters = try!(stmt.query(&[])).iter().map(|row|
            CharacterMinimal::from_sql(row)).collect();
        info!("loaded {} characters", self.characters.len());

        Ok(())
    }
}
