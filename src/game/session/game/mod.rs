pub mod chunk;
mod handlers;

use std::collections::HashMap;
use shared::session::SessionBase;
use time;
use character::{Character, CharacterMinimal};

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
    base: SessionBase,
    account: Option<AccountData>,
    state: GameState,
}
