use mio::{Token, EventLoop, EventSet, Handler};
use super::{Msg, Listener};
use pool;

impl Handler for Listener {
    type Timeout = ();
    type Message = Msg;

    fn ready(&mut self, event_loop: &mut EventLoop<Listener>, token: Token,
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
                    let _ = self.pool.send(pool::Msg::SessionRemove(token));
                }
            }
        }
    }

    fn notify(&mut self, event_loop: &mut EventLoop<Listener>, msg: Msg) {
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
        }
    }
}
