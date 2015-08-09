use mio;
use std::sync::mpsc::Sender;
use std::io;
use std::io::Cursor;
use net::Token;
use net;
use pool;
use super::chunk::Chunk;

pub trait Session : Drop {
    type C: Chunk;

    fn new(Token, Sender<pool::Msg>, mio::Sender<net::Msg>) -> Option<Self>;
    fn get_handler(u16) -> (fn(&mut Self, &Self::C, Cursor<Vec<u8>>) -> io::Result<()>);

    fn unhandled(&mut self, _: &Self::C, _: Cursor<Vec<u8>>) -> io::Result<()> {
        Ok(())
    }

    fn handle_packet(&mut self, area: &Self::C, id: u16, data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        Self::get_handler(id)(self, area, data)
    }
}
