pub mod data;

use session::{auth, game};
use std::collections::{HashSet, HashMap};
use shared::net::{Token, SessionEvent};
use shared::chunk;
use shared::HashBiMap;
use eventual::Async;
use character::CharacterMinimal;
use self::data::GameServerData;
use shared::server::ServerBase;
use std::sync::Mutex;

pub type Sender = chunk::Sender<Server>;

lazy_static! { pub static ref SYNC_SERVER: Mutex<Option<GameServerData>> = Mutex::new(None); }
thread_local!(pub static SERVER: GameServerData = SYNC_SERVER.lock().unwrap().clone().unwrap());

pub struct Server {
    base: ServerBase<game::Session, game::chunk::ChunkImpl,
        auth::Session, auth::chunk::ChunkImpl>,

    // an in-game session can be identified by its character id
    session_characters: HashBiMap<i32, Token>,

    // and also by its nickname
    session_nicknames: HashBiMap<String, Token>,

    characters: HashMap<i32, CharacterMinimal>,
    chunk_areas: HashMap<i16, usize>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            base: ServerBase::new(),
            session_characters: HashBiMap::new(),
            session_nicknames: HashBiMap::new(),
            characters: HashMap::new(),
            chunk_areas: HashMap::new(),
        }
    }

    pub fn auth_event(&mut self, evt: SessionEvent) {
        self.base.secondary_event(evt);
    }

    pub fn game_event(&mut self, evt: SessionEvent) {
        if let SessionEvent::Disconnect(tok) = evt {
            let _ = self.session_characters.inv_remove(&tok);
            let _ = self.session_nicknames.inv_remove(&tok);
        }

        self.base.main_event(evt);
    }
}

pub fn start_queue_timer(sender: &Sender) {
    let tx = sender.clone();
    chunk::send(sender, move |server| {
        server.base.queue_timer.interval_ms(2000).each(move |()| {
            chunk::send(&tx, move |server| {
                for chunk in &server.base.main_chunks {
                    chunk::send(chunk, |chunk| {
                        game::chunk::update_queue(chunk);
                    });
                }
            })
        }).fire();
    });
}

pub fn add_chunk(sender: &Sender, chunk: game::chunk::Sender, areas: HashSet<i16>) {
    chunk::send(sender, move |server| {
        let len = server.base.main_chunks.len();
        for a in areas {
            let _ = server.chunk_areas.insert(a, len);
        }
        server.base.main_chunks.push(chunk)
    });
}

pub fn set_auth_chunk(sender: &Sender, chunk: auth::chunk::Sender) {
    chunk::send(sender, move |server| {
        server.base.secondary_chunk = Some(chunk);
    });
}

pub fn teleport<F>(sender: &Sender, tok: Token, area_id: i16, job: F)
    where F: FnOnce(&mut game::chunk::Chunk) + Send + 'static {

    chunk::send(sender, move |server| {
        let chunk = server.chunk_areas.get(&area_id).unwrap();
        let _ = server.base.session_chunks.remove(&tok);
        let _ = server.base.session_chunks.insert(tok, *chunk);
        chunk::send(&server.base.main_chunks[*chunk], job);
    });
}

pub fn identification_success<F>(sender: &Sender, tok: Token, id: i32, job: F)
    where F: FnOnce(&mut game::Session, bool, HashMap<i32, CharacterMinimal>)
    + Send + 'static {

    chunk::send(sender, move |server| {
        let already = server.base.session_ids.contains_key(&id);
        let mut characters = HashMap::new();

        if !already {
            let _ = server.base.session_ids.insert(id, tok);

            characters = server.characters.iter().filter_map(|ch| {
                if ch.1.account_id() == id {
                    return Some((*ch.0, ch.1.clone()));
                }
                None
            }).collect();
        }

        server.base.session_callback(tok, move |session, _| job(session, already, characters))
    });
}

pub fn character_selection_success<F>(sender: &Sender, tok: Token, ch_id: i32,
    nickname: String, job: F)
    where F: FnOnce(&mut game::Session, &mut game::chunk::ChunkImpl) + Send + 'static {

    chunk::send(sender, move |server| {
        let _ = server.session_characters.insert(ch_id, tok);
        let _ = server.session_nicknames.insert(nickname, tok);

        server.base.session_callback(tok, move |session, mut chunk| job(session, &mut *chunk))
    });
}
