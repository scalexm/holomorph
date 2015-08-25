use server;
use std::sync::Arc;
use config::Config;
use shared::{net, pool};

#[derive(Clone)]
pub struct GameServerData {
    pub handler: server::Sender,
    pub io_loop: net::Sender,
    pub cnf: Arc<Config>,
}

impl GameServerData {
    pub fn new(handler: server::Sender, io_loop: net::Sender,
        cnf: Config) -> GameServerData {

        GameServerData {
            handler: handler,
            io_loop: io_loop,
            cnf: Arc::new(cnf),
        }
    }

    pub fn shutdown(&self) {
        let _ = self.io_loop.send(net::Msg::Shutdown);
        let _ = self.handler.send(pool::Msg::Shutdown);
    }
}
