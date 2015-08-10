use shared::pool;
use std::sync::mpsc::Sender;
use mio;
use shared::net;
use shared::database;
use config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthServer {
    pub pool: Sender<pool::Msg>,
    pub io_loop: mio::Sender<net::Msg>,
    pub db: Sender<database::Thunk>,
    pub key: Arc<Vec<u8>>,
    pub patch: Arc<Vec<u8>>,
    pub cnf: Arc<Config>,
}


impl AuthServer {
    pub fn new(pool: Sender<pool::Msg>, io_loop: mio::Sender<net::Msg>,
        db: Sender<database::Thunk>, key: Vec<u8>, patch: Vec<u8>,
        cnf: Config) -> AuthServer {

            AuthServer {
                pool: pool,
                io_loop: io_loop,
                db: db,
                key: Arc::new(key),
                patch: Arc::new(patch),
                cnf: Arc::new(cnf),
            }
    }

    pub fn shutdown(&self) {
        let _ = self.io_loop.send(net::Msg::Shutdown);
        let _ = self.pool.send(pool::Msg::Shutdown);
    }
}
