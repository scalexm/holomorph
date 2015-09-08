pub mod chunk;

use net::Token;
use std::io::{self, Cursor};
use self::chunk::Ref;

// base Session class
pub struct SessionBase {
    pub token: Token,
    pub address: String,
}

pub trait Session<U>: Sized {
    fn new(SessionBase) -> Self;

    fn get_handler<'a>(u16) -> (fn(&mut Self, Ref<'a, Self, U>, Cursor<Vec<u8>>)
        -> io::Result<()>);

    fn unhandled<'a>(&mut self, _: Ref<'a, Self, U>, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        Ok(())
    }

    fn close<'a>(self, Ref<'a, Self, U>);
}

impl SessionBase {
    fn new(token: Token, address: String) -> Self {
        debug!("{:?} connected", token);

        SessionBase {
            token: token,
            address: address,
        }
    }
}

impl Drop for SessionBase {
    fn drop(&mut self) {
        debug!("{:?} logout", self.token);
    }
}
