extern crate shared;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;
extern crate rustc_serialize;

mod session;
mod handlers;
mod chunk;
mod config;

use shared::net::{Msg, Listener};
use mio::EventLoop;
use chunk::Chunk;
use shared::pool::SessionPool;
use std::thread;
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;
use std::env;

fn load(path: &str) -> Arc<Vec<u8>> {
    let mut data = Vec::new();
    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    Arc::new(data)
}

fn main() {
    env_logger::init().unwrap();

    let cnf = config::from_file(&env::args()
        .nth(1)
        .unwrap_or("config.json".to_string()));

    let mut pool = SessionPool::<Chunk>::new();
    let tx = pool.channel();

    let key = load(&cnf.key_path);
    let patch = load(&cnf.patch_path);
    for _ in (0..cnf.num_threads) {
        pool.run_chunk(Chunk::new(tx.clone(), key.clone(), patch.clone()));
    }

    thread::spawn(move || pool.run());

    let mut event_loop = EventLoop::new().unwrap();
    let mut server = Listener::new(&mut event_loop, &cnf.bind_address, tx).unwrap();

    let tx = event_loop.channel();
    thread::spawn(move || {
        io::stdin().read_line(&mut String::new())
            .ok()
            .expect("failed to read line");
        let _ = tx.send(Msg::Shutdown);
    });

    event_loop.run(&mut server).unwrap();
}
