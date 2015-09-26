use std::collections::HashMap;
use character::Character;
use shared::protocol::*;
use shared::protocol::variants::GameRolePlayActorInformationsVariant;
use shared::protocol::messages::game::context::roleplay::{GameRolePlayShowActorMessage,
    MapComplementaryInformationsDataMessage};
use shared::protocol::messages::game::context::{GameContextRemoveElementMessage,
    GameMapMovementMessage};
use server::SERVER;

pub enum Actor {
    Character(Character),
    _Lol, // debug
}

impl Actor {
    pub fn id(&self) -> i32 {
        match *self {
            Actor::Character(ref ch) => ch.minimal().id(),
            _ => unreachable!(),
        }
    }

    pub fn as_actor(&self) -> GameRolePlayActorInformationsVariant {
        match *self {
            Actor::Character(ref ch) => ch.as_actor().into(),
            _ => unreachable!(),
        }
    }

    pub fn into_character(self) -> Character {
        match self {
            Actor::Character(ch) => ch,
            _ => panic!("error: actor {} is not a Character", self.id()),
        }
    }

    pub fn as_character(&self) -> &Character {
        match *self {
            Actor::Character(ref ch) => ch,
            _ => panic!("error: actor {} is not a Character", self.id()),
        }
    }

    pub fn as_mut_character(&mut self) -> &mut Character {
        match *self {
            Actor::Character(ref mut ch) => ch,
            _ => panic!("error: actor {} is not a Character", self.id()),
        }
    }

    pub fn set_cell_id(&mut self, cell: i16) {
        match *self {
            Actor::Character(ref mut ch) => ch.set_cell_id(cell),
            _ => unreachable!(),
        }
    }

    pub fn is_character(&self) -> bool {
        if let Actor::Character(..) = *self {
            return true;
        }
        false
    }
}

pub struct Map {
    id: i32,
    area_id: i16,
    actors: HashMap<i32, Actor>, // a Map owns its actors
}

macro_rules! get_characters {
    ($map: expr) => {
        $map.actors.values()
            .filter_map(|a| if a.is_character() { Some(a.as_character()) } else { None })
    };
}

impl Map {
    pub fn new(id: i32, area_id: i16) -> Self {
        Map {
            id: id,
            area_id: area_id,
            actors: HashMap::new(),
        }
    }

    pub fn get_complementary_informations(&self) -> MapComplementaryInformationsDataMessage {
        MapComplementaryInformationsDataMessage {
            sub_area_id: VarShort(SERVER.with(|s| s.maps.get(&self.id).unwrap().sub_area_id())),
            map_id: self.id,
            houses: Vec::new(),
            actors: self.actors.values().map(|a| a.as_actor()).collect(),
            interactive_elements: Vec::new(),
            stated_elements: Vec::new(),
            obstacles: Vec::new(),
            fights: Vec::new(),
        }
    }

    pub fn area_id(&self) -> i16 {
        self.area_id
    }

    pub fn get_actor(&self, id: i32) -> Option<&Actor> {
        self.actors.get(&id)
    }

    pub fn get_mut_actor(&mut self, id: i32) -> Option<&mut Actor> {
        self.actors.get_mut(&id)
    }

    pub fn add_actor(&mut self, actor: Actor) {
        let id = actor.id();
        let buf = GameRolePlayShowActorMessage {
            informations: actor.as_actor(),
        }.as_packet().unwrap();

        self.send(buf);
        let _ = self.actors.insert(id, actor);
    }

    pub fn remove_actor(&mut self, id: i32) -> Option<Actor> {
        let actor = self.actors.remove(&id);
        if let Some(ref actor) = actor {
            let buf = GameContextRemoveElementMessage {
                id: actor.id(),
            }.as_packet().unwrap();

            self.send(buf);
        }
        actor
    }

    pub fn start_move_actor(&self, id: i32, movs: Vec<i16>) {
        if !self.actors.contains_key(&id) {
            return ();
        }

        let buf = GameMapMovementMessage {
            key_movements: movs,
            actor_id: id,
        }.as_packet().unwrap();

        self.send(buf);
    }

    pub fn end_move_actor(&self, id: i32) {
        if !self.actors.contains_key(&id) {
            return ();
        }

        let buf = GameMapMovementMessage {
            key_movements: Vec::new(),
            actor_id: id,
        }.as_packet().unwrap();

        self.send(buf);
    }

    pub fn teleport(&mut self, id: i32, cell: i16) -> bool {
        match self.remove_actor(id) {
            Some(mut actor) => {
                actor.set_cell_id(cell);
                self.add_actor(actor);
                true
            },
            None => false,
        }
    }

    pub fn send(&self, buf: Vec<u8>) {
        for ch in get_characters!(self) {
            let buf = buf.clone();
            write!(SERVER, ch.session(), buf);
        }
    }
}
