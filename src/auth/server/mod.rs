pub mod data;

use shared::server::ServerBase;
use shared::chunk;
use session::{auth, game};
use shared::net::{Token, SessionEvent};
use shared::HashBiMap;
use eventual::Async;
use shared::protocol::Protocol;
use shared::protocol::holomorph::DisconnectPlayerMessage;
use std::sync::Mutex;
use self::data::AuthServerData;
use shared::protocol::enums::server_status;

pub type Sender = chunk::Sender<Server>;

lazy_static! { pub static ref SYNC_SERVER: Mutex<Option<AuthServerData>> = Mutex::new(None); }
thread_local!(pub static SERVER: AuthServerData = SYNC_SERVER.lock().unwrap().clone().unwrap());

pub struct Server {
    base: ServerBase<auth::Session, auth::chunk::ChunkImpl,
        game::Session, game::chunk::ChunkImpl>,
    game_session_ids: HashBiMap<i16, Token>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            base: ServerBase::new(),
            game_session_ids: HashBiMap::new(),
        }
    }

    pub fn game_event(&mut self, evt: SessionEvent) {
        if let SessionEvent::Disconnect(tok) = evt {
            match self.game_session_ids.inv_remove(&tok) {
                Some(id) => self.update_game_server(id,
                   server_status::OFFLINE, String::new(), 0),
                None => (),
           }
        }

        self.base.secondary_event(evt);
    }

    pub fn auth_event(&mut self, evt: SessionEvent) {
        self.base.main_event(evt);
    }

    fn update_game_server(&mut self, id: i16, status: i8,
        ip: String, port: i16) {

        for chunk in &self.base.main_chunks {
            let ip = ip.clone();
            chunk::send(chunk, move |chunk| {
                auth::chunk::update_game_server(chunk, id, status, ip, port);
            });
        }
    }
}

pub fn start_queue_timer(sender: &Sender) {
    let tx = sender.clone();
    chunk::send(sender, move |server| {
        server.base.queue_timer.interval_ms(2000).each(move |()| {
            chunk::send(&tx, move |server| {
                for chunk in &server.base.main_chunks {
                    chunk::send(chunk, |chunk| {
                        auth::chunk::update_queue(chunk);
                    });
                }
            })
        }).fire();
    });
}

pub fn add_chunk(sender: &Sender, chunk: auth::chunk::Sender) {
    chunk::send(sender, move |server| {
        server.base.main_chunks.push(chunk)
    });
}

pub fn set_game_chunk(sender: &Sender, chunk: game::chunk::Sender) {
    chunk::send(sender, move |server| {
        server.base.secondary_chunk = Some(chunk);
    });
}

pub fn identification_success<F>(sender: &Sender, tok: Token, id: i32,
    already_logged: i16, job: F)
    where F: for<'a> FnOnce(&mut auth::Session, &auth::chunk::ChunkImpl, bool)
    + Send + 'static {

    chunk::send(sender, move |server| {
        let already = server.base.session_ids.insert(id, tok);
        if let Some(session) = already {
            close!(SERVER, session);
        }

        if let Some(tok) = server.game_session_ids.get(&already_logged) {
            let buf = DisconnectPlayerMessage {
                id: id,
            }.as_packet().unwrap();
            write!(SERVER, *tok, buf);
        }

        server.base.session_callback(tok,
            move |session, chunk| job(session, &*chunk, already.is_some()))
    });
}

pub fn register_game_server<F>(sender: &Sender, tok: Token, id: i16, state: i8,
    ip: String, port: i16, job: F)
    where F: FnOnce(&mut game::Session, Option<i16>) + Send + 'static {

    chunk::send(sender, move |server| {
        let mut server_id = None;
        if !server.game_session_ids.contains_key(&id) {
            let _ = server.game_session_ids.insert(id, tok);
            server.update_game_server(id, state, ip, port);
            server_id = Some(id);
        }

        chunk::send(server.base.secondary_chunk.as_ref().unwrap(), move |chunk| {
            chunk.session_callback(tok, move |session, _| {
                job(session, server_id)
            });
        });
    });
}

pub fn update_game_server(sender: &Sender, id: i16, state: i8, ip: String, port: i16) {
    chunk::send(sender, move |server| {
        server.update_game_server(id, state, ip, port)
    });
}
