pub mod chunk;
mod handlers;

use shared::session::SessionBase;

pub struct Session {
    base: SessionBase,
    server_id: Option<i16>,
    salt: String,
    ip: String,
    port: i16,
}
