use mio::{Token, EventSet, Handler};
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
                    event_loop.deregister(&self.connections[token].socket).unwrap();
                    let _ = self.connections.remove(token).unwrap();
                    let _ = self
                        .handler
                        .send(pool::NetMsg::SessionDisconnect(token).into());
                }
            }
        }
    }

    fn notify(&mut self, event_loop: &mut EventLoop<C>, msg: Msg) {
        match msg {
            Msg::Shutdown => {
                event_loop.shutdown();
            }

            Msg::Write(tok, buf) => {
                if let Some(conn) = self.connections.get_mut(tok) {
                    conn.push(buf, false, event_loop)
                }
            }

            Msg::WriteAndClose(tok, buf) => {
                if let Some(conn) = self.connections.get_mut(tok) {
                    conn.push(buf, true, event_loop)
                }
            }

            Msg::Close(tok) => {
                if let Some(conn) = self.connections.get_mut(tok) {
                    let _ = conn.socket.shutdown(Shutdown::Both);
                }
            }
        }
    }
}
