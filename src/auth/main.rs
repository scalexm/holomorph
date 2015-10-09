#[macro_use]
extern crate shared;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
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

// for loading dofus public key and authentification patch
fn load(path: &str) -> Vec<u8> {
    let mut data = Vec::new();
    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    data
}

struct ProgramState {
    io_loop: EventLoop<server::Server>,
    network_handler: net::Handler<server::Server>,
    shutdown_tx: mpsc::Sender<()>,
    shutdown_rx: mpsc::Receiver<()>,
    join_handles: LinkedList<JoinHandle<()>>,
}

fn start(args: &str) -> ProgramState {
    let cnf = shared::config::from_file::<Config>(&args);
    let mut join_handles = LinkedList::new();

    let db = database::spawn_threads(cnf.database_threads,
                                     &cnf.database_uri,
                                     &mut join_handles);
    let key = load(&cnf.key_path);
    let patch = load(&cnf.patch_path);

    let mut io_loop = EventLoop::new().unwrap();

    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    let server_data = {
        let mut conn = database::connect(&cnf.database_uri);
        let server = chunk::run(server::Server::new(), &mut join_handles);

        let mut server_data = AuthServerData::new(server,
                                                  io_loop.channel(),
                                                  db,
                                                  key,
                                                  patch,
                                                  cnf,
                                                  shutdown_tx.clone());

        if let Err(err) = server_data.load(&mut conn) {
            panic!("loading failed: {}", err);
        }

        server_data
    };

    *SYNC_SERVER.lock().unwrap() = Some(server_data.clone());

    assert!(server_data.cnf.server_threads >= 1);
    for _ in (0..server_data.cnf.server_threads) {
        let tx = chunk::run(auth::chunk::new(), &mut join_handles);
        server::add_chunk(&server_data.server, tx);
    }

    let tx = chunk::run(game::chunk::new(), &mut join_handles);
    server::set_game_chunk(&server_data.server, tx);

    let mut network_handler = net::Handler::new(server_data.server.clone());

    network_handler.add_callback(&mut io_loop, &server_data.cnf.bind_address,
                                 server::Server::auth_event, CallbackType::Listen);

    network_handler.add_callback(&mut io_loop, &server_data.cnf.game_bind_address,
                                 server::Server::game_event, CallbackType::Listen);

    server::start_queue_timer(&server_data.server);

    ProgramState {
        io_loop: io_loop,
        network_handler: network_handler,
        shutdown_tx: shutdown_tx,
        shutdown_rx: shutdown_rx,
        join_handles: join_handles,
    }
}

fn main() {
    env_logger::init().unwrap();
    let time_point = time::precise_time_ns();

    let args = env::args().nth(1)
                          .unwrap_or("auth_config.json".to_string());
    let mut state = start(&args);

    let shutdown_tx = state.shutdown_tx;
    thread::spawn(move || {
        io::stdin().read_line(&mut String::new())
            .ok()
            .expect("failed to read line");
        let _ = shutdown_tx.send(());
    });

    let io_tx = state.io_loop.channel();
    let shutdown_rx = state.shutdown_rx;
    thread::spawn(move || {
        let _ = shutdown_rx.recv();
        let _ = io_tx.send(net::Msg::Shutdown);
        *SYNC_SERVER.lock().unwrap() = None;
    });

    info!("server loaded in {} ms", (time::precise_time_ns() - time_point) / 1000000);

    let mut io_loop = state.io_loop;
    io_loop.run(&mut state.network_handler).unwrap();

    // joining all threads so that all callbacks (especially database ones) can be called
    for handle in state.join_handles {
        let _ = handle.join();
    }
}
