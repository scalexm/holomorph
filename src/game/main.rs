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
extern crate protocol;
extern crate eventual;

mod config;
mod session;
mod server;
mod character;
mod stats;
mod map;

use config::Config;
use shared::net::{self, EventLoop, CallbackType};
use shared::chunk;
use std::thread::{self, JoinHandle};
use std::env;
use std::io;
use server::data::GameServerData;
use shared::database;
use server::SYNC_SERVER;
use std::collections::{HashSet, LinkedList};
use session::{auth, game};
use std::cmp::Ordering;
use std::sync::mpsc;

struct ProgramState {
    io_loop: EventLoop<server::Server>,
    network_handler: net::Handler<server::Server>,
    shutdown_tx: mpsc::Sender<()>,
    shutdown_rx: mpsc::Receiver<()>,
    join_handles: LinkedList<JoinHandle<()>>,
}

fn start(args: &str) -> ProgramState {
    let cnf = shared::config::from_file::<Config>(args);
    let mut join_handles = LinkedList::new();

    let db = database::spawn_threads(cnf.database_threads,
                                     &cnf.database_uri,
                                     &mut join_handles);
    let auth_db = database::spawn_threads(cnf.auth_database_threads,
                                          &cnf.auth_database_uri,
                                          &mut join_handles);

    let mut io_loop = EventLoop::new().unwrap();

    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    let server_data = {
        let mut conn = database::connect(&cnf.database_uri);
        let mut server = server::Server::new();

        if let Err(err) = server.load(&mut conn) {
            panic!("loading failed: {}", err);
        }

        let server = chunk::run(server, &mut join_handles);

        let mut server_data = GameServerData::new(server, io_loop.channel(), cnf, db, auth_db,
                                                  shutdown_tx.clone());

        if let Err(err) = server_data.load(&mut conn) {
            panic!("loading failed: {}", err);
        }

        server_data
    };

    *SYNC_SERVER.lock().unwrap() = Some(server_data.clone());

    // dividing all areas between chunks
    let _ = {
        let mut chunk_areas = (0..server_data.cnf.server_threads).map(|_| HashSet::new())
                                                                 .collect::<Vec<_>>();
        let len = chunk_areas.len();
        let mut sorted_areas: Vec<(i16, i16)> = server_data.areas
                                                           .values()
                                                           .map(|a| (a.id(), a.priority()))
                                                           .collect();
        sorted_areas.sort_by(|&(_, p1), &(_, p2)| {
            if p1 > p2 { Ordering::Less } else { Ordering::Greater }
        });

        for i in 0..sorted_areas.len() {
            let _ = chunk_areas[i % len].insert(sorted_areas[i].0);
        }

        for set in chunk_areas {
            let tx = chunk::run(game::chunk::new(set.clone(), &server_data), &mut join_handles);
            server::add_chunk(&server_data.server, tx, set);
        }
    };

    let tx = chunk::run(auth::chunk::new(), &mut join_handles);
    server::set_auth_chunk(&server_data.server, tx);

    let mut network_handler = net::Handler::new(server_data.server.clone());
    let bind_address = format!("{}:{}", &server_data.cnf.bind_ip, &server_data.cnf.bind_port);

    network_handler.add_callback(&mut io_loop, &bind_address, server::Server::game_event,
                                 CallbackType::Listen);

    network_handler.add_callback(&mut io_loop, &server_data.cnf.auth_address,
                                 server::Server::auth_event, CallbackType::Connect);

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
                          .unwrap_or("game_config.json".to_string());
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
