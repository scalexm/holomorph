use shared::chunk;
use shared::session;
use super::{Session, CharacterRef};
use map::{Actor, Map};
use std::collections::{HashSet, HashMap};
use server::data::GameServerData;
use character::Character;
use server::{self, SERVER};
use shared::protocol::*;
use shared::protocol::messages::game::context::roleplay::CurrentMapMessage;

pub type Chunk = session::chunk::Chunk<Session, ChunkImpl>;
pub type Ref<'a> = session::chunk::Ref<'a, Session, ChunkImpl>;
pub type Sender = chunk::Sender<Chunk>;

pub struct ChunkImpl {
    pub maps: HashMap<i32, Map>,
    areas: HashSet<i16>,
}

impl ChunkImpl {
    fn new(areas: HashSet<i16>, server: &GameServerData) -> Self {
        let mut maps = HashMap::new();
        for &a_id in &areas {
            for s_id in server.sub_areas
                .values()
                .filter_map(|s| if s.area_id() == a_id { Some(s.id()) }
                    else { None }) {

                for m_id in server.maps
                    .values()
                    .filter_map(|m| if m.sub_area_id() == s_id { Some(m.id()) }
                        else { None }) {

                    let _ = maps.insert(m_id, Map::new(m_id, a_id));
                }
            }
        }

        ChunkImpl {
            maps: maps,
            areas: areas,
        }
    }
}

pub fn new(areas: HashSet<i16>, server: &GameServerData) -> Chunk {
    Chunk::new(ChunkImpl::new(areas, server))
}

pub fn update_queue(chunk: &Chunk) {
    for session in chunk.sessions.values() {
        session.update_queue();
    }
}

pub fn teleport_character(chunk: &mut Chunk, mut ch: Character, map_id: i32, cell_id: i16) {
    if let Some(mut map) = chunk.impl_.maps.get_mut(&map_id) {
        ch.set_cell_id(cell_id);
        let tok = ch.session();
        map.add_actor(Actor::Character(ch));

        let buf = CurrentMapMessage {
            map_id: map_id,
            map_key: "bite".to_string(),
        }.as_packet().unwrap();

        write!(SERVER, tok, buf);
        return ();
    }

    SERVER.with(|s| {
        let map = s.maps.get(&map_id).unwrap();
        let area_id = s.sub_areas
            .get(&map.sub_area_id())
            .unwrap()
            .area_id();

        let tok = ch.session();
        let session = chunk.sessions.remove(&tok).unwrap();

        server::teleport(&s.server, tok, area_id, move |dest_chunk| {
            let _ = dest_chunk.sessions.insert(tok, session);
            let _ = teleport_character(dest_chunk, ch, map_id, cell_id);
        });
    })
}

pub fn teleport<'a>(mut chunk: Ref<'a>, ch_ref: &mut CharacterRef, mut map_id: i32, cell_id: i16)
    -> bool {

    let ch = {
        let map = chunk.maps.get_mut(&ch_ref.map_id).unwrap();

        if ch_ref.map_id == map_id {
            return map.teleport(ch_ref.id, cell_id);
        }

        if !SERVER.with(|s| {
            if let Some(map_data) = s.maps.get(&map_id) {
                if map_data.is_bad() {
                    map_id = map_data.relative();
                }
                return true;
            }
            false
        }) {

            return false;
        }

        match map.remove_actor(ch_ref.id).map(|ch| ch.into_character()) {
            Some(ch) => ch,
            None => return false,
        }
    };

    ch_ref.map_id = map_id;
    chunk.eventually(move |chunk| teleport_character(chunk, ch, map_id, cell_id));
    true
}
