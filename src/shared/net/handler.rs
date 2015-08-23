use mio::{EventSet, Handler, PollOpt};
use mio::tcp::Shutdown;
use net::*;
use pool;

impl<C: pool::Chunk> Handler for NetworkHandler<C> {
    type Timeout = ();
    type Message = Msg;

    fn ready(&mut self, event_loop: &mut EventLoop<C>, tok: Token,
        events: EventSet) {

        match tok {
            // one of our listeners
            Token(t) if t < 10 => {
                if let Err(err) = self.handle_server_event(event_loop, tok, events) {
                    error!("accept failed: {}", err);
                }
            }

            // one of our connections
            _ => {
                if let Err(_) = self.handle_client_event(event_loop, tok, events) {
                    // if an error occurs, we disconnect the session (typically: EOF)
                    let _ = event_loop.deregister(&self.connections[tok].socket);

                    let cb = self.listeners[self.connections[tok].listener_token].callback;
                    pool::execute(&self.handler, move |handler|
                        cb(handler, SessionEvent::Disconnect(tok)));

                    let _ = self.connections.remove(tok).unwrap();
                }
            }
        }
    }

    fn notify(&mut self, event_loop: &mut EventLoop<C>, msg: Msg) {
        let mut close = false;
        if let Msg::WriteAndClose(..) = msg {
            close = true;
        }

        match msg {
            Msg::Shutdown => {
                event_loop.shutdown();
            }

            Msg::Write(tok, buf) | Msg::WriteAndClose(tok, buf) => {
                let _ = self.connections.get_mut(tok).map(|conn| {
                    conn.push(buf, close);

                    event_loop.reregister(&conn.socket, tok,
                        EventSet::readable() | EventSet::writable(),
                        PollOpt::level()).unwrap();
                });
            }

            Msg::Close(tok) => {
                let _ = self.connections.get_mut(tok).map(|conn|
                    conn.socket.shutdown(Shutdown::Both));
            }
        }
    }
}
