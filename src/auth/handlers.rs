use mio::Sender;
use shared::net::Msg;
use std::io;
use std::io::Cursor;
use shared::io::{ReadExt, WriteExt};
use shared::protocol::*;
use shared::protocol::connection::*;
use session::Session;
use chunk::Chunk;

impl Session {
    pub fn handle_identification(&mut self, _: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        println!("lol");
        Ok(())
    }
}
