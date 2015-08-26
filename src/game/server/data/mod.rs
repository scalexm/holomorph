use server;
use std::sync::Arc;
use config::Config;
use shared::{net, pool};
use shared::database;
use postgres::Result;
use character::CharacterMinimal;
use server::Handler;

#[derive(Clone)]
pub struct GameServerData {
    pub handler: server::Sender,
    pub io_loop: net::Sender,
    pub cnf: Arc<Config>,
    pub auth_db: database::Sender,
}

impl GameServerData {
    pub fn new(handler: server::Sender, io_loop: net::Sender,
        cnf: Config, auth_db: database::Sender) -> GameServerData {

        GameServerData {
            handler: handler,
            io_loop: io_loop,
            cnf: Arc::new(cnf),
            auth_db: auth_db,
        }
    }

    pub fn shutdown(&self) {
        let _ = self.io_loop.send(net::Msg::Shutdown);
        let _ = self.handler.send(pool::Msg::Shutdown);
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
