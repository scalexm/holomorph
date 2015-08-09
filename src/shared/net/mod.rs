use mio::{EventLoop, EventSet, PollOpt};
use mio::tcp::TcpListener;
use mio::util::Slab;
use std::io;
use std::sync::mpsc::Sender;

mod connection;
mod handler;
pub use mio::Token;
use self::connection::Connection;

const SERVER: Token = Token(0);

pub struct Listener {
    pool: Sender<::pool::Msg>,
    server: TcpListener,
    connections: Slab<Connection>,
}

#[derive(Debug)]
pub enum Msg {
    Shutdown,
    Write(Token, Vec<u8>),
    Close(Token),
}

impl Listener {
    pub fn new(event_loop: &mut EventLoop<Listener>, address: &str,
        pool: Sender<::pool::Msg>) -> io::Result<Listener> {

        let address = address.parse().unwrap();
        let server = try!(TcpListener::bind(&address));

        try!(event_loop.register_opt(&server, SERVER, EventSet::readable(),
            PollOpt::edge()));

        info!("ready to listen on {:?}", address);
        let slab = Slab::new_starting_at(Token(1), 1024);

        Ok(Listener {
            pool: pool,
            server: server,
            connections: slab,
        })
    }

    fn handle_server_event(&mut self, event_loop: &mut EventLoop<Listener>,
        events: EventSet) -> io::Result<()> {

        assert!(events.is_readable());

        match try!(self.server.accept()) {
            Some(socket) => {
                let pool = self.pool.clone();
                let token = self.connections
                    .insert_with(move |token| Connection::new(socket, token, pool))
                    .unwrap();

                self.pool
                    .send(::pool::Msg::SessionCreate(token, event_loop.channel()))
                    .unwrap();

                event_loop.register_opt(&self.connections[token].socket,
                    token,
                    EventSet::readable() | EventSet::writable(),
                    PollOpt::level())
            }

            None => Ok(()),
        }
    }

    fn handle_client_event(&mut self, token: Token, events: EventSet)
        -> io::Result<()> {

        if events.is_readable() {
            try!(self.connections[token].readable());
        }

        if events.is_writable() {
            try!(self.connections[token].writable());
        }

        Ok(())
    }
}
