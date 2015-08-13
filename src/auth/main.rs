extern crate shared;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;
extern crate rustc_serialize;
extern crate postgres;
extern crate crypto;
extern crate time;
extern crate eventual;

mod session;
mod config;
mod server;
mod chunk;

use shared::net::{EventLoop, Listener};
use shared::pool;
use std::thread;
use std::fs::File;
use std::io::{self, Read};
use std::env;
use shared::database;
use server::data::AuthServerData;
use eventual::*;

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

    let db = pool::run_chunk(database::connect(&cnf.database_uri));
    let key = load(&cnf.key_path);
    let patch = load(&cnf.patch_path);

    let mut io_loop = EventLoop::new().unwrap();
    let handler = pool::run_chunk(server::Handler::new(io_loop.channel()));

    let mut server_data = AuthServerData::new(handler.clone(), io_loop.channel(),
        db, key, patch, cnf);

    if let Err(err) = server_data.load() {
        panic!("loading failed: {}", err);
    }

    for _ in (0..server_data.cnf.num_threads) {
        let tx = pool::run_chunk(session::Chunk::new(server_data.clone()));
        server::add_chunk(&handler, tx);
    }

    let mut listener = match Listener::new(&mut io_loop, &server_data.cnf.bind_address,
        handler.clone()) {

        Ok(listener) => listener,
        Err(err) => panic!("listen failed: {}", err),
    };

    thread::spawn(move || {
        io::stdin().read_line(&mut String::new())
            .ok()
            .expect("failed to read line");
        server_data.shutdown();
    });

    server::start_queue_timer(&handler);
    io_loop.run(&mut listener).unwrap();
}
