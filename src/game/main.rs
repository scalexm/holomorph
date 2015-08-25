extern crate shared;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustc_serialize;

mod config;
mod session;
mod server;

use config::Config;
use shared::net::{EventLoop, NetworkHandler, CallbackType};
use shared::pool;
use std::thread;
use std::env;
use std::io;
use std::fmt::Write;
use server::data::GameServerData;

fn main() {
    env_logger::init().unwrap();

    let cnf = shared::config::from_file::<Config>(&env::args()
        .nth(1)
        .unwrap_or("game_config.json".to_string()));

    let mut io_loop = EventLoop::new().unwrap();
    let handler = pool::run_chunk(server::Handler::new(io_loop.channel()));
    let mut network_handler = NetworkHandler::new(handler.clone());

    let mut server_data = GameServerData::new(handler.clone(), io_loop.channel(), cnf);

    let tx = pool::run_chunk(session::game::Chunk::new(server_data.clone()));
    server::add_chunk(&handler, tx);

    let tx = pool::run_chunk(session::auth::Chunk::new(server_data.clone()));
    server::set_auth_chunk(&handler, tx);

    let mut bind_address = String::new();
    let _ = write!(&mut bind_address, "{}:{}", &server_data.cnf.bind_ip,
        &server_data.cnf.bind_port);

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

    io_loop.run(&mut network_handler).unwrap();
}
