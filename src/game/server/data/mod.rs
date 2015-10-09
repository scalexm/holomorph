mod map;

use std::sync::Arc;
use config::Config;
use shared::{net, database};
use postgres::{Connection, Result};
use character::CharacterMinimal;
use self::map::*;
use std::collections::HashMap;
use server::{self, Server};
use std::sync::mpsc;

#[derive(Clone)]
pub struct GameServerData {
    pub server: server::Sender,
    pub io_loop: net::Sender,
    pub cnf: Arc<Config>,
    pub auth_db: database::Sender,
    pub db: database::Sender,
    pub maps: Arc<HashMap<i32, MapData>>,
    pub sub_areas: Arc<HashMap<i16, SubAreaData>>,
    pub areas: Arc<HashMap<i16, AreaData>>,
    shutdown: mpsc::Sender<()>,
}

impl GameServerData {
    pub fn new(server: server::Sender, io_loop: net::Sender, cnf: Config, db: database::Sender,
               auth_db: database::Sender, shutdown: mpsc::Sender<()>) -> Self {

        GameServerData {
            server: server,
            io_loop: io_loop,
            cnf: Arc::new(cnf),
            auth_db: auth_db,
            db: db,
            maps: Arc::new(HashMap::new()),
            sub_areas: Arc::new(HashMap::new()),
            areas: Arc::new(HashMap::new()),
            shutdown: shutdown,
        }
    }

    pub fn load(&mut self, conn: &mut Connection) -> Result<()> {
        let stmt = try!(conn.prepare("SELECT * FROM areas"));
        self.areas = Arc::new(try!(stmt.query(&[])).iter().map(|row|
            AreaData::from_sql(row)).collect());
        info!("loaded {} areas", self.areas.len());

        let stmt = try!(conn.prepare("SELECT * FROM sub_areas"));
        self.sub_areas = Arc::new(try!(stmt.query(&[])).iter().map(|row|
            SubAreaData::from_sql(&*self.areas, row)).collect());
        info!("loaded {} sub areas", self.sub_areas.len());

        let stmt = try!(conn.prepare("SELECT * FROM map_positions JOIN maps
            ON map_positions.id = maps.id"));
        self.maps = Arc::new(try!(stmt.query(&[])).iter().map(|row|
            MapData::from_sql(&*self.sub_areas, row)).collect());
        info!("loaded {} maps", self.maps.len());

        Ok(())
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown.send(());
    }
}

impl Server {
    pub fn load(&mut self, conn: &mut Connection) -> Result<()> {
        let stmt = try!(conn.prepare("SELECT * FROM character_minimals"));
        self.characters = try!(stmt.query(&[])).iter().map(|row|
            CharacterMinimal::from_sql(row)).collect();
        info!("loaded {} characters", self.characters.len());

        Ok(())
    }
}
