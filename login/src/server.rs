use crate::config::Config;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use hashbrown::HashMap;
use log::info;
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use std::sync::atomic::AtomicU8;
use holomorph::{ContextualError, WithContext};

pub struct Server {
    pub config: Config,
    pub database_pool: Pool<ConnectionManager<PgConnection>>,
    pub public_key: Vec<u8>,
    pub private_key: Rsa<Private>,
    pub game_servers: HashMap<i16, GameServer>,
}

impl Server {
    pub fn load(&mut self) -> Result<(), ContextualError> {
        use database::game_servers::dsl::*;

        let conn = self
            .database_pool
            .get()
            .context("could not get a database connection from the pool")?;

        self.game_servers = game_servers
            .load::<(i16, String, i16)>(&conn)
            .context("could not load game servers")?
            .into_iter()
            .map(GameServer::new)
            .collect();
        info!("{} game servers loaded", self.game_servers.len());

        Ok(())
    }
}

#[derive(Debug)]
pub struct GameServer {
    id: i16,
    host: String,
    port: i16,
    status: AtomicU8,
}

impl GameServer {
    fn new((id, host, port): (i16, String, i16)) -> (i16, Self) {
        let gs = Self {
            id,
            host,
            port,
            status: AtomicU8::new(0),
        };
        (id, gs)
    }

    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn status(&self) -> u8 {
        self.status.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> i16 {
        self.port
    }
}
