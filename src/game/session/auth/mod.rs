pub mod chunk;
mod handlers;

use shared::session;

pub type Session = session::Session<SessionImpl>;

struct SessionImpl;
