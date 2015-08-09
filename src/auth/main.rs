extern crate shared;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;

mod session;
mod handlers;
mod chunk;

use shared::net::Listener;
use mio::EventLoop;
use chunk::Chunk;
use shared::pool::SessionPool;
use std::thread;


fn main() {
    env_logger::init().unwrap();
    let mut pool = SessionPool::<Chunk>::new();
    let tx = pool.channel();
    pool.run_area(Chunk::new(tx.clone()));
    thread::spawn(move || pool.run());

    let mut event_loop = EventLoop::new().unwrap();
    let mut server = Listener::new(&mut event_loop, "127.0.0.1:2000", tx).unwrap();
    event_loop.run(&mut server).unwrap();
}
