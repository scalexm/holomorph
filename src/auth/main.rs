#![feature(plugin, custom_attribute, custom_derive)]
#![plugin(codegen, diesel_codegen)]

#[macro_use] extern crate shared;
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate diesel;
extern crate env_logger;
extern crate time;
extern crate eventual;
extern crate rustc_serialize;
extern crate rand;
extern crate protocol;
extern crate openssl;

mod session;
mod config;
mod server;

use shared::net::{self, EventLoop, CallbackType};
use shared::chunk;
use std::thread::{self, JoinHandle};
use std::fs::File;
use std::io::{self, Read};
use std::env;
use shared::database;
use server::data::AuthServerData;
use config::Config;
use session::{auth, game};
use server::SYNC_SERVER;
use std::collections::LinkedList;
use std::sync::mpsc;
use openssl::crypto::pkey::{EncryptionPadding, PKey};

struct ProgramState {
    io_loop: EventLoop<server::Server>,
    network_handler: net::Handler<server::Server>,
    join_handles: LinkedList<JoinHandle<()>>,
}

fn start(args: &str) -> ProgramState {
    let cnf = shared::config::from_file::<Config>(&args);
    let mut join_handles = LinkedList::new();

    let db = database::spawn_threads(
        cnf.database_threads,
        &cnf.database_uri,
        &mut join_handles
    );

    let mut io_loop = EventLoop::new().unwrap();

    let mut sign_key_raw = Vec::new();
    File::open(&cnf.key_path).unwrap().read_to_end(&mut sign_key_raw).unwrap();
    let mut sign_key = PKey::new();
    sign_key.load_priv(&sign_key_raw[0..]);

    let mut key = PKey::new();
    key.gen(2048);

    let server_data = {
        let mut conn = database::connect(&cnf.database_uri);
        let server = chunk::run(server::Server::new(), &mut join_handles);

        let mut server_data = AuthServerData::new(
            server,
            io_loop.channel(),
            db,
            sign_key.private_encrypt_with_padding(
                &key.save_pub()[0..],
                EncryptionPadding::PKCS1v15
            ),
            key.save_priv(),
            cnf
        );

        server_data.load(&mut conn);
        server_data
    };

    *SYNC_SERVER.lock().unwrap() = Some(server_data.clone());

    assert!(server_data.cnf.server_threads >= 1);
    for _ in 0..server_data.cnf.server_threads {
        let tx = chunk::run(auth::chunk::new(), &mut join_handles);
        server::add_chunk(&server_data.server, tx);
    }

    let tx = chunk::run(game::chunk::new(), &mut join_handles);
    server::set_game_chunk(&server_data.server, tx);

    let mut network_handler = net::Handler::new(server_data.server.clone(), 2);

    network_handler.add_callback(
        &mut io_loop,
        &server_data.cnf.bind_address,
        server::Server::auth_event,
        CallbackType::Listen
    );

    network_handler.add_callback(
        &mut io_loop,
        &server_data.cnf.game_bind_address,
        server::Server::game_event,
        CallbackType::Listen
    );

    server::start_queue_timer(&server_data.server);

    ProgramState {
        io_loop: io_loop,
        network_handler: network_handler,
        join_handles: join_handles,
    }
}

fn main() {
    env_logger::init().unwrap();
    let time_point = time::precise_time_ns();

    let args = env::args().nth(1)
                          .unwrap_or("auth_config.json".to_string());
    let mut state = start(&args);

    info!("server loaded in {} ms", (time::precise_time_ns() - time_point) / 1000000);

    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    thread::spawn(move || {
        println!("press [Enter] to exit");
        io::stdin().read_line(&mut String::new())
                   .ok()
                   .expect("failed to read line");
        let _ = shutdown_tx.send(());
    });

    let io_tx = state.io_loop.channel();
    thread::spawn(move || {
        let _ = shutdown_rx.recv();
        let _ = io_tx.send(net::Msg::Shutdown);
        *SYNC_SERVER.lock().unwrap() = None;
    });

    let mut io_loop = state.io_loop;
    io_loop.run(&mut state.network_handler).unwrap();

    // joining all threads so that all callbacks (especially database ones) can be called
    for handle in state.join_handles {
        let _ = handle.join();
    }
}
