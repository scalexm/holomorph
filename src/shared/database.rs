use postgres::{Connection, SslMode};
use pool;

pub fn connect(uri: &str) -> Connection {
    match Connection::connect(uri, &SslMode::None) {
        Ok(conn) => conn,
        Err(err) => panic!("database connection failed: {}", err),
    }
}

impl pool::Chunk for Connection {
    fn process_net_msg(&mut self, _: pool::NetMsg) {
        unreachable!()
    }
}

pub type Sender = pool::Sender<Connection>;
