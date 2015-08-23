extern crate shared;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate postgres;
extern crate crypto;
extern crate time;
extern crate eventual;
extern crate rustc_serialize;
extern crate rand;

mod session;
mod config;
mod server;

use shared::net::{EventLoop, NetworkHandler};
use shared::pool;
use std::thread;
use std::fs::File;
use std::io::{self, Read};
use std::env;
use shared::database;
use server::data::AuthServerData;
use config::Config;

fn load(path: &str) -> Vec<u8> {
    let mut data = Vec::new();
    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    data
}

fn main() {
    env_logger::init().unwrap();

    let cnf = shared::config::from_file::<Config>(&env::args()
        .nth(1)
        .unwrap_or("config.json".to_string()));

    let db = database::spawn_threads(cnf.database_threads, &cnf.database_uri);
    let key = load(&cnf.key_path);
    let patch = load(&cnf.patch_path);

    let mut io_loop = EventLoop::new().unwrap();
    let handler = pool::run_chunk(server::Handler::new(io_loop.channel()));
    let mut network_handler = NetworkHandler::new(handler.clone());

    let mut server_data = AuthServerData::new(handler.clone(), io_loop.channel(),
        db, key, patch, cnf);

    if let Err(err) = server_data.load() {
        panic!("loading failed: {}", err);
    }

    assert!(server_data.cnf.server_threads >= 1);
    for _ in (0..server_data.cnf.server_threads) {
        let tx = pool::run_chunk(session::auth::Chunk::new(server_data.clone()));
        server::add_chunk(&handler, tx);
    }

    let tx = pool::run_chunk(session::game::Chunk::new(server_data.clone()));
    server::set_game_chunk(&handler, tx);

    network_handler.add_listener(&mut io_loop, &server_data.cnf.bind_address,
        server::Handler::auth_event);

    network_handler.add_listener(&mut io_loop, &server_data.cnf.game_bind_address,
        server::Handler::game_event);

    thread::spawn(move || {
        io::stdin().read_line(&mut String::new())
            .ok()
            .expect("failed to read line");
        server_data.shutdown();
    });

    server::start_queue_timer(&handler);
    //server::update_game_server(&handler, 1, 3, "127.0.0.1".to_string(), 5556);
    io_loop.run(&mut network_handler).unwrap();
}
