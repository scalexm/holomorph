use mio::{Token, EventSet, Handler, PollOpt};
use mio::tcp::Shutdown;
use super::{Msg, Listener, EventLoop};
use pool;

impl<C: pool::Chunk> Handler for Listener<C> {
    type Timeout = ();
    type Message = Msg;

    fn ready(&mut self, event_loop: &mut EventLoop<C>, token: Token,
        events: EventSet) {

        match token {
            super::SERVER => {
                if let Err(err) = self.handle_server_event(event_loop, events) {
                    event_loop.shutdown();
                    error!("accept failed: {}", err);
                }
            }

            _ => {
                if let Err(_) = self.handle_client_event(token, event_loop, events) {
                    let _ = event_loop.deregister(&self.connections[token].socket);
                    let _ = self.connections.remove(token).unwrap();
                    let _ = self
                        .handler
                        .send(pool::NetMsg::SessionDisconnect(token).into());
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
