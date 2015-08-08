extern crate shared;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;

mod session;

use shared::net::Server;
use mio::EventLoop;
use session::Session;

fn main() {
    env_logger::init().unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    let mut server = Server::<Session>::new(&mut event_loop, "127.0.0.1:2000").unwrap();
    event_loop.run(&mut server).unwrap();
}
