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
mod game_session;
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

    let db = pool::run_chunk(database::connect(&cnf.database_uri));
    let key = load(&cnf.key_path);
    let patch = load(&cnf.patch_path);

    let mut auth_loop = EventLoop::new().unwrap();
    let mut game_loop = EventLoop::new().unwrap();
    let handler = pool::run_chunk(server::Handler::new(auth_loop.channel()));
    let fwd_handler = pool::run_chunk(server::ForwardingHandler::new(handler.clone()));

    let mut server_data = AuthServerData::new(handler.clone(), auth_loop.channel(),
        game_loop.channel(), db, key, patch, cnf);

    if let Err(err) = server_data.load() {
        panic!("loading failed: {}", err);
    }

    for _ in (0..server_data.cnf.num_threads) {
        let tx = pool::run_chunk(session::Chunk::new(server_data.clone()));
        server::add_chunk(&handler, tx);
    }

    let tx = pool::run_chunk(game_session::Chunk::new(server_data.clone()));
    server::set_game_chunk(&handler, tx);

    let mut auth_listener = match Listener::new(&mut auth_loop, &server_data.cnf.bind_address,
        handler.clone()) {

        Ok(listener) => listener,
        Err(err) => panic!("listen failed: {}", err),
    };

    let mut game_listener = match Listener::new(&mut game_loop,
        &server_data.cnf.game_bind_address, fwd_handler.clone()) {

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
    thread::spawn(move || game_loop.run(&mut game_listener).unwrap());
    auth_loop.run(&mut auth_listener).unwrap();
}
