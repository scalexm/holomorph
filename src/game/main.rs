#[macro_use]
extern crate shared;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
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
use server::data::GameServerData;
use shared::database;
use server::SYNC_SERVER;
use std::collections::LinkedList;
use session::{auth, game};

fn main() {
    env_logger::init().unwrap();
    let time_point = time::precise_time_ns();

    let cnf = shared::config::from_file::<Config>(&env::args()
        .nth(1)
        .unwrap_or("game_config.json".to_string()));

    let mut join_handles = LinkedList::new();

    let db = database::spawn_threads(cnf.database_threads,
            &cnf.database_uri, &mut join_handles);

    let auth_db = database::spawn_threads(cnf.auth_database_threads,
            &cnf.auth_database_uri, &mut join_handles);

    let mut io_loop = EventLoop::new().unwrap();

    let mut server_data;
    {
        let mut conn = database::connect(&cnf.database_uri);
        let mut server = server::Server::new();

        if let Err(err) = server.load(&mut conn) {
            panic!("loading failed: {}", err);
        }

        let server = chunk::run(server, &mut join_handles);

        server_data = GameServerData::new(server, io_loop.channel(), cnf,
            db, auth_db);

        if let Err(err) = server_data.load(&mut conn) {
            panic!("loading failed: {}", err);
        }
    }

    *SYNC_SERVER.lock().unwrap() = Some(server_data.clone());

    let tx = chunk::run(game::chunk::new(), &mut join_handles);
    server::add_chunk(&server_data.server, tx);

    let tx = chunk::run(auth::chunk::new(), &mut join_handles);
    server::set_auth_chunk(&server_data.server, tx);

    let mut network_handler = net::Handler::new(server_data.server.clone());
    let bind_address = format!("{}:{}", &server_data.cnf.bind_ip,
        &server_data.cnf.bind_port);

    network_handler.add_callback(&mut io_loop, &bind_address,
        server::Server::game_event, CallbackType::Listen);

    network_handler.add_callback(&mut io_loop, &server_data.cnf.auth_address,
        server::Server::auth_event, CallbackType::Connect);

    server::start_queue_timer(&server_data.server);

    thread::spawn(move || {
        io::stdin().read_line(&mut String::new())
            .ok()
            .expect("failed to read line");
        server_data.shutdown();
    });

    info!("server loaded in {} ms", (time::precise_time_ns() - time_point) / 1000000);

    io_loop.run(&mut network_handler).unwrap();

    for handle in join_handles {
        let _ = handle.join();
    }
}
