extern crate shared;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;
extern crate rustc_serialize;
extern crate postgres;

mod session;
mod handlers;
mod chunk;
mod config;
mod server;

use shared::net::Listener;
use mio::EventLoop;
use chunk::Chunk;
use shared::pool::SessionPool;
use std::thread;
use std::fs::File;
use std::io::{self, Read};
use std::env;
use shared::database;
use server::AuthServer;

fn load(path: &str) -> Vec<u8> {
    let mut data = Vec::new();
    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    data
}

fn main() {
    env_logger::init().unwrap();

    let cnf = config::from_file(&env::args()
        .nth(1)
        .unwrap_or("config.json".to_string()));

    let db = database::async_connect(&cnf.database_uri);
    let key = load(&cnf.key_path);
    let patch = load(&cnf.patch_path);

    let mut io_loop = EventLoop::new().unwrap();
    let mut pool = SessionPool::new();

    let server = AuthServer::new(pool.channel(), io_loop.channel(), db,
        key, patch, cnf);

    for _ in (0..server.cnf.num_threads) {
        pool.run_chunk(Chunk::new(server.clone()));
    }

    let mut listener = Listener::new(&mut io_loop, &server.cnf.bind_address,
        pool.channel()).unwrap();

    thread::spawn(move || {
        io::stdin().read_line(&mut String::new())
            .ok()
            .expect("failed to read line");
        server.shutdown();
    });

    thread::spawn(move || pool.run());
    io_loop.run(&mut listener).unwrap();
}
