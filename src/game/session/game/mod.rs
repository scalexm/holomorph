macro_rules! get_mut_character {
    ($ch: ident, $chunk: ident) => {
        $chunk.maps
            .get_mut(&$ch.map_id).unwrap()
            .get_mut_actor($ch.id).unwrap()
            .as_mut_character()
    };
}

macro_rules! get_character {
    ($ch: ident, $chunk: ident) => {
        $chunk.maps
            .get(&$ch.map_id).unwrap()
            .get_actor($ch.id).unwrap()
            .as_character()
    };
}

pub mod chunk;
mod handlers;

use std::collections::{HashMap, HashSet};
use shared;
use time;
use character::{Character, CharacterMinimal};
use shared::protocol::variants::{FriendInformationsVariant, IgnoredInformationsVariant};

pub struct CharacterRef {
    id: i32,
    map_id: i32,
    movements: Option<Vec<i16>>,
}

struct AccountData {
    id: i32,
    nickname: String,
    secret_answer: String,
    level: i8,
    subscription_end: i64,
    last_connection: i64,
    last_ip: String,
    friends: HashSet<String>,
    ignored: HashSet<String>,
}

impl AccountData {
    fn is_subscriber(&self) -> bool {
        self.subscription_end > time::get_time().sec
    }
}

enum GameState {
    None,
    TicketQueue(isize, isize),
    CharacterSelection(HashMap<i32, CharacterMinimal>), // at this point, self.account is some
    GameQueue(isize, isize),
    SwitchingContext(i32, Character),
    InContext(CharacterRef),
}

impl GameState {
    fn is_none(&self) -> bool {
        match *self {
            GameState::None => true,
            _ => false,
        }
    }
}

pub struct Session {
    base: shared::session::SessionBase,
    account: Option<AccountData>,
    state: GameState,

    last_sales_chat_request: i64,
    last_seek_chat_request: i64,

    friends: HashMap<String, FriendInformationsVariant>,
    ignored: HashMap<String, IgnoredInformationsVariant>,
}
