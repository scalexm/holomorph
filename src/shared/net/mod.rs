mod connection;
mod handler;

use mio::{self, EventSet, PollOpt};
use mio::tcp::TcpListener;
use mio::util::Slab;
use std::io;
use net::connection::Connection;
use pool;

pub use mio::Token;

const SERVER: Token = Token(0);

pub struct Listener<C: pool::Chunk> {
    handler: pool::Sender<C>,
    server: TcpListener,
    connections: Slab<Connection<C>>,
}

pub enum Msg {
    Shutdown,
    Write(Token, Vec<u8>),
    WriteAndClose(Token, Vec<u8>),
}

pub type Sender = mio::Sender<Msg>;
pub type EventLoop<C> = mio::EventLoop<Listener<C>>;

impl<C: pool::Chunk> Listener<C> {
    pub fn new(event_loop: &mut EventLoop<C>, address: &str,
        handler: pool::Sender<C>) -> io::Result<Listener<C>> {

        let address = match address.parse() {
            Ok(addr) => addr,
            Err(_) => panic!("failed to parse address"),
        };

        let server = try!(TcpListener::bind(&address));

        try!(event_loop.register_opt(&server, SERVER, EventSet::readable(),
            PollOpt::edge()));

        info!("ready to listen on {:?}", address);
        let slab = Slab::new_starting_at(Token(1), 1024);

        Ok(Listener {
            handler: handler,
            server: server,
            connections: slab,
        })
    }

    fn handle_server_event(&mut self, event_loop: &mut EventLoop<C>,
        events: EventSet) -> io::Result<()> {

        assert!(events.is_readable());

        match try!(self.server.accept()) {
            Some(socket) => {
                let handler = self.handler.clone();
                let token = self.connections
                    .insert_with(move |token| Connection::new(socket, token, handler))
                    .unwrap();

                self.handler
                    .send(pool::NetMsg::SessionConnect(token).into())
                    .unwrap();

                event_loop.register_opt(&self.connections[token].socket,
                    token,
                    EventSet::readable(),
                    PollOpt::level())
            }

            None => Ok(()),
        }
    }

    fn handle_client_event(&mut self, token: Token, event_loop: &mut EventLoop<C>,
        events: EventSet) -> io::Result<()> {

        if events.is_readable() {
            try!(self.connections[token].readable());
        }

        if events.is_writable() {
            try!(self.connections[token].writable(event_loop));
        }

        Ok(())
    }
}
