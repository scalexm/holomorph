use mio::{Sender, Token};
use std::io;
use std::io::Cursor;
use net::server::Msg;

pub trait Session {
    fn new(Token, Sender<Msg>) -> Self;
    fn handle_packet(&mut self, u16, Cursor<Vec<u8>>) -> io::Result<()>;
}
