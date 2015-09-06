
#[macro_use]
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

use shared::net::{self, EventLoop, CallbackType};
use shared::chunk;
use std::thread;
use std::fs::File;
use std::io::{self, Read};
use std::env;
use shared::database;
use server::data::AuthServerData;
use config::Config;
use session::{auth, game};

// for loading dofus public key and authentification patch
fn load(path: &str) -> Vec<u8> {
    let mut data = Vec::new();
    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    data
}

fn main() {
    env_logger::init().unwrap();
    let time_point = time::precise_time_ns();

    let cnf = shared::config::from_file::<Config>(&env::args()
        .nth(1)
        .unwrap_or("auth_config.json".to_string()));

    let mut join_handles = Vec::new();

    let db = database::spawn_threads(cnf.database_threads, &cnf.database_uri,
        &mut join_handles);
    let key = load(&cnf.key_path);
    let patch = load(&cnf.patch_path);

    let mut io_loop = EventLoop::new().unwrap();
    let handler = chunk::run(server::Handler::new(io_loop.channel()),
        &mut join_handles);
    let mut network_handler = net::Handler::new(handler.clone());

    let mut server_data = AuthServerData::new(handler.clone(), io_loop.channel(),
        db, key, patch, cnf);

    if let Err(err) = server_data.load() {
        panic!("loading failed: {}", err);
    }

    assert!(server_data.cnf.server_threads >= 1);
    for _ in (0..server_data.cnf.server_threads) {
        let tx = chunk::run(auth::chunk::new(server_data.clone()),
            &mut join_handles);
        server::add_chunk(&handler, tx);
    }

    let tx = chunk::run(game::chunk::new(server_data.clone()),
        &mut join_handles);
    server::set_game_chunk(&handler, tx);

    network_handler.add_callback(&mut io_loop, &server_data.cnf.bind_address,
        server::Handler::auth_event, CallbackType::Listen);

    network_handler.add_callback(&mut io_loop, &server_data.cnf.game_bind_address,
        server::Handler::game_event, CallbackType::Listen);

    thread::spawn(move || {
        io::stdin().read_line(&mut String::new())
            .ok()
            .expect("failed to read line");
        server_data.shutdown();
    });

    info!("server loaded in {} ms", (time::precise_time_ns() - time_point) / 1000000);

    server::start_queue_timer(&handler);
    io_loop.run(&mut network_handler).unwrap();

    // joining all threads so that all callbacks (especially database ones) can be called
    for handle in join_handles {
        let _ = handle.join();
    }
}
