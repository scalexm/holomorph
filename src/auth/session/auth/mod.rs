pub mod chunk;
mod handlers;

use shared::session::SessionBase;
use std::collections::HashMap;
use time;

struct AccountData {
    id: i32,
    account: String,
    nickname: String,
    secret_question: String,
    level: i8,
    subscription_end: i64,
    subscription_elapsed: i64,
    creation_date: i64,
    character_counts: HashMap<i16, i8>,
    already_logged: i16,
    last_server: i16,
}

impl AccountData {
    fn is_subscriber(&self) -> bool {
        self.subscription_end > time::get_time().sec
    }
}

enum QueueState {
    None,
    Some(isize, isize),
}

impl QueueState {
    fn is_none(&self) -> bool {
        match *self {
            QueueState::None => true,
            _ => false,
        }
    }
}

pub struct Session {
    base: SessionBase,
    account: Option<AccountData>,
    queue_state: QueueState,
    custom_identification: bool,
    aes_key: Vec<u8>,
}
