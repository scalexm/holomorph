pub mod chunk;
mod handlers;

use std::collections::HashMap;
use shared::session;
use time;
use character::{Character, CharacterMinimal};

pub type Session = session::Session<SessionImpl>;

enum QueueState {
    None,
    SomeTicket(isize, isize),
    SomeGame(isize, isize),
}

impl QueueState {
    fn is_none(&self) -> bool {
        match *self {
            QueueState::None => true,
            _ => false,
        }
    }
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

struct SessionImpl {
    queue_state: QueueState,
    account: Option<AccountData>,
    characters: HashMap<i32, CharacterMinimal>,
    current_character: Option<Character>,
}
