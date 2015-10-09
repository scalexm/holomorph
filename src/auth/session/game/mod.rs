pub mod chunk;
mod handlers;

use shared;

pub struct Session {
    base: shared::session::SessionBase,
    server_id: Option<i16>,
    salt: String,
    ip: String,
    port: i16,
}
