pub mod chunk;

use net::Token;
use std::ops::{Deref, DerefMut};
use std::io::{self, Cursor};
use self::chunk::Chunk;

// base Session class
pub struct Session<T> {
    pub token: Token,
    pub address: String,
    pub impl_: T, // custom fields
}

impl<T> Deref for Session<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.impl_
    }
}

impl<T> DerefMut for Session<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.impl_
    }
}

pub trait SessionImpl: Sized {
    type Chunk;

    fn new(Token, &Self::Chunk) -> Self;

    fn get_handler(u16) -> (fn(&mut Session<Self>, &Self::Chunk,
        Cursor<Vec<u8>>) -> io::Result<()>);

    fn unhandled(&mut Session<Self>, _: &Self::Chunk, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        Ok(())
    }

    fn close(Session<Self>, &Self::Chunk);
}

impl<T: SessionImpl<Chunk = Chunk<T, U>>, U> Session<T> {
    pub fn new(token: Token, address: String, chunk: &Chunk<T, U>) -> Self {
        debug!("{:?} connected", token);

        Session {
            token: token,
            address: address,
            impl_: T::new(token, chunk),
        }
    }

    pub fn handle_packet(&mut self, chunk: &Chunk<T, U>, id: u16, data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        T::get_handler(id)(self, chunk, data)
    }

    pub fn close(self, chunk: &Chunk<T, U>) {
        T::close(self, chunk)
    }
}


impl<T> Drop for Session<T> {
    fn drop(&mut self) {
        debug!("{:?} logout", self.token);
    }
}
