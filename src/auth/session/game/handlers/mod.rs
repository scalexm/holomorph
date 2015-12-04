mod misc;

use std::io::{Result, Cursor};
use super::Session;
use super::chunk::{Ref, ChunkImpl};
use protocol::Protocol;
use protocol::holomorph::*;
use shared;
use server::SERVER;
use rand::{self, Rng};

impl shared::session::Session<ChunkImpl> for Session {
    fn new(base: shared::session::SessionBase) -> Self {
        let salt = rand::thread_rng().gen_ascii_chars().take(32).collect::<String>();

        let buf = HelloMessage {
            salt: salt.clone(),
        }.as_packet().unwrap();

        write!(SERVER, base.token, buf);

        Session {
            base: base,
            server_id: None,
            salt: salt,
            ip: String::new(),
            port: 0,
        }
    }

    fn handle<'a>(&mut self, chunk: Ref<'a>, id: i16, mut data: Cursor<Vec<u8>>) -> Result<()> {
        handle!(self, chunk, id, data)
    }

    fn close<'a>(self, _: Ref<'a>) { }
}
