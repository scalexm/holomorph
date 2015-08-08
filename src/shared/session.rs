use mio::{Sender, Token};
use std::io;
use std::io::Cursor;
use net::server::Msg;

pub trait Session {
    fn new(Token, Sender<Msg>) -> Self;

    fn get_handler(u16) -> (fn(&mut Self, Cursor<Vec<u8>>) -> io::Result<()>);

    fn unhandled(&mut self, _: Cursor<Vec<u8>>) -> io::Result<()> {
        Ok(())
    }

    fn handle_packet(&mut self, id: u16, data: Cursor<Vec<u8>>) -> io::Result<()> {
        Self::get_handler(id)(self, data)
    }
}
