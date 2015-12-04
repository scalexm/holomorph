mod misc;

use super::Session;
use super::chunk::{ChunkImpl, Ref};
use protocol::*;
use protocol::holomorph::*;
use shared::{self, crypt};
use std::io::{Result, Cursor};
use server::{self, SERVER};

impl shared::session::Session<ChunkImpl> for Session {
    fn new(base: shared::session::SessionBase) -> Self {
        Session {
            base: base,
        }
    }

    fn handle<'a>(&mut self, chunk: Ref<'a>, id: i16, mut data: Cursor<Vec<u8>>) -> Result<()> {
        handle!(self, chunk, id, data)
    }

    fn close<'a>(self, _: Ref<'a>) {
        error!("FATAL ERROR: lost connection with auth server");
        SERVER.with(|s| s.shutdown());
    }
}
