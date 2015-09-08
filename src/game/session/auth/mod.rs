pub mod chunk;
mod handlers;

use shared::session::SessionBase;

pub struct Session {
    base: SessionBase,
}
