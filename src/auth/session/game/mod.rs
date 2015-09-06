pub mod chunk;
mod handlers;

use shared::session;

pub type Session = session::Session<SessionImpl>;

struct SessionImpl {
    server_id: Option<i16>,
    salt: String,
    ip: String,
    port: i16,
}
