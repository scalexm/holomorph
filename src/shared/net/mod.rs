mod connection;
mod handler;

use mio::{self, EventSet, PollOpt};
use mio::tcp::TcpListener;
use mio::util::Slab;
use std::io::{self, Cursor};
use net::connection::Connection;
use pool;

pub use mio::Token;

pub enum Msg {
    // received by NetworkHandler
    Shutdown,
    Write(Token, Vec<u8>),
    WriteAndClose(Token, Vec<u8>),
    Close(Token),

    // sent by NetworkHandler
    SessionConnect(Token),
    SessionPacket(Token, u16, Cursor<Vec<u8>>),
    SessionDisconnect(Token),
}

struct Listener<C> {
    socket: TcpListener,
    callback: fn(&mut C, Msg),
}

pub type Sender = mio::Sender<Msg>;
pub type EventLoop<C> = mio::EventLoop<NetworkHandler<C>>;

pub struct NetworkHandler<C: pool::Chunk> {
    handler: pool::Sender<C>,
    listeners: Slab<Listener<C>>,
    connections: Slab<Connection>,
}

impl<C: pool::Chunk> NetworkHandler<C> {
    fn add_listener_with_result(&mut self, event_loop: &mut EventLoop<C>, address: &str,
        cb: fn(&mut C, Msg)) -> io::Result<()> {

            let address = match address.parse() {
                Ok(addr) => addr,
                Err(_) => panic!("failed to parse address"),
            };

            let socket = try!(TcpListener::bind(&address));
            let tok = self.listeners.insert(Listener {
                socket: socket,
                callback: cb,
            }).ok().unwrap();

            try!(event_loop.register_opt(&self.listeners[tok].socket, tok,
                EventSet::readable(), PollOpt::edge()));

            info!("ready to listen on {:?}", address);
            Ok(())
    }

    pub fn add_listener(&mut self, event_loop: &mut EventLoop<C>, address: &str,
        cb: fn(&mut C, Msg)) {

        if let Err(err) = self.add_listener_with_result(event_loop, address, cb) {
            panic!("listen failed {}", err);
        }
    }

    pub fn new(handler: pool::Sender<C>) -> NetworkHandler<C> {
        NetworkHandler {
            handler: handler,
            listeners: Slab::new(10), // we keep 10 tokens for the Listeners
            connections: Slab::new_starting_at(Token(10), 65535),
        }
    }

    fn handle_server_event(&mut self, event_loop: &mut EventLoop<C>, tok: Token,
        events: EventSet) -> io::Result<()> {

        assert!(events.is_readable());

        match try!(self.listeners[tok].socket.accept()) {
            Some(socket) => {
                let client_tok = self.connections
                    .insert(Connection::new(socket, tok))
                    .ok()
                    .unwrap();

                let cb = self.listeners[tok].callback;
                pool::execute(&self.handler, move |handler| {
                    cb(handler, Msg::SessionConnect(client_tok))
                });

                event_loop.register_opt(&self.connections[client_tok].socket,
                    client_tok,
                    EventSet::readable(),
                    PollOpt::level())
            }

            None => Ok(()),
        }
    }

    fn handle_client_event(&mut self, event_loop: &mut EventLoop<C>, tok: Token,
        events: EventSet) -> io::Result<()> {

        if events.is_readable() {
            if let Some(packet) = try!(self.connections[tok].readable()) {
                let cb = self.listeners[self.connections[tok].listener_token].callback;
                pool::execute(&self.handler, move |handler| {
                    cb(handler, Msg::SessionPacket(tok, packet.0, packet.1));
                });
            }
        }

        if events.is_writable() {
            if try!(self.connections[tok].writable()) {
                event_loop.reregister(&self.connections[tok].socket, tok,
                    EventSet::readable(),
                    PollOpt::level()).unwrap();
            }
        }

        Ok(())
    }
}
