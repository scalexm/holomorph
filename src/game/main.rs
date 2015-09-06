
#[macro_use]
extern crate shared;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustc_serialize;
extern crate postgres;
extern crate time;
extern crate eventual;

mod config;
mod session;
mod server;
mod character;
mod stats;

use config::Config;
use shared::net::{self, EventLoop, CallbackType};
use shared::chunk;
use std::thread;
use std::env;
use std::io;
use std::fmt::Write;
use server::data::GameServerData;
use shared::database;

fn main() {
    env_logger::init().unwrap();
    let time_point = time::precise_time_ns();

    let cnf = shared::config::from_file::<Config>(&env::args()
        .nth(1)
        .unwrap_or("game_config.json".to_string()));

    let mut join_handles = Vec::new();

    let db = database::spawn_threads(cnf.database_threads,
            &cnf.database_uri, &mut join_handles);

    let auth_db = database::spawn_threads(cnf.auth_database_threads,
            &cnf.auth_database_uri, &mut join_handles);

    let mut io_loop = EventLoop::new().unwrap();

    let mut handler = server::Handler::new(io_loop.channel());
    if let Err(err) = handler.load(&cnf.database_uri) {
        panic!("loading failed: {}", err);
    }

    let handler = chunk::run(handler, &mut join_handles);

    let mut network_handler = net::Handler::new(handler.clone());

    let mut server_data = GameServerData::new(handler.clone(), io_loop.channel(), cnf,
        db, auth_db);

    if let Err(err) = server_data.load() {
        panic!("loading failed: {}", err);
    }

    let tx = chunk::run(session::game::chunk::new(server_data.clone()),
        &mut join_handles);
    server::add_chunk(&handler, tx);

    let tx = chunk::run(session::auth::chunk::new(server_data.clone()),
        &mut join_handles);
    server::set_auth_chunk(&handler, tx);

    let mut bind_address = String::new();
    write!(&mut bind_address, "{}:{}", &server_data.cnf.bind_ip,
        &server_data.cnf.bind_port).unwrap();

    network_handler.add_callback(&mut io_loop, &bind_address,
        server::Handler::game_event, CallbackType::Listen);

    network_handler.add_callback(&mut io_loop, &server_data.cnf.auth_address,
        server::Handler::auth_event, CallbackType::Connect);

    thread::spawn(move || {
        io::stdin().read_line(&mut String::new())
            .ok()
            .expect("failed to read line");
        server_data.shutdown();
    });

    info!("server loaded in {} ms", (time::precise_time_ns() - time_point) / 1000000);

    server::start_queue_timer(&handler);
    io_loop.run(&mut network_handler).unwrap();

    for handle in join_handles {
        let _ = handle.join();
    }
}
