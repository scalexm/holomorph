use mio::tcp::*;
use std::io;
use mio::util::Slab;
use mio;
use super::connection::Connection;
use ::session::Session;

const SERVER: mio::Token = mio::Token(0);

pub struct Server<S: Session> {
    server: TcpListener,
    connections: Slab<Connection<S>>,
}

#[derive(Debug)]
pub enum Msg {
    Shutdown,
    Write(mio::Token, Vec<u8>),
}

impl<S: Session> Server<S> {
    pub fn new(event_loop: &mut mio::EventLoop<Server<S>>, address: &str)
        -> io::Result<Server<S>> {

        let address = address.parse().unwrap();
        let server = try!(TcpListener::bind(&address));

        try!(event_loop.register_opt(&server, SERVER, mio::EventSet::readable(),
            mio::PollOpt::edge()));

        info!("ready to listen on {:?}", address);
        let slab = Slab::new_starting_at(mio::Token(1), 1024);

        Ok(Server {
            server: server,
            connections: slab,
        })
    }

    pub fn handle_event(&mut self, token: mio::Token, events: mio::EventSet)
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

impl<S: Session> mio::Handler for Server<S> {
    type Timeout = ();
    type Message = Msg;

    fn ready(&mut self, event_loop: &mut mio::EventLoop<Server<S>>, token: mio::Token,
        events: mio::EventSet) {

        match token {
            SERVER => {
                assert!(events.is_readable());

                match self.server.accept() {
                    Ok(Some(socket)) => {
                        let token = self.connections
                            .insert_with(|token|
                                Connection::new(socket, token, event_loop.channel()))
                            .unwrap();

                        debug!("connected {:?}", token);

                        event_loop.register_opt(
                            &self.connections[token].socket,
                            token,
                            mio::EventSet::readable() | mio::EventSet::writable(),
                            mio::PollOpt::level()).unwrap();
                    }

                    Ok(None) => {
                        debug!("the server socket wasn't actually ready");
                    }

                    Err(e) => {
                        error!("server.accept() errored: {}", e);
                        event_loop.shutdown();
                    }
                }
            }
            
            _ => {
                if let Err(_) = self.handle_event(token, events) {
                    event_loop.deregister(&self.connections[token].socket).unwrap();
                    let _ = self.connections.remove(token).unwrap();

                    debug!("logout {:?}", token);
                }
            }
        }
    }

    fn notify(&mut self, event_loop: &mut mio::EventLoop<Server<S>>, msg: Msg) {
        match msg {
            Msg::Shutdown => event_loop.shutdown(),
            Msg::Write(tok, buf) => self.connections[tok].write(buf),
        }
    }
}
