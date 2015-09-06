use std::io::{self, Cursor};
use super::{Session, SessionImpl};
use super::chunk::Chunk;
use shared::net::{Token, Msg};
use shared::protocol::*;
use shared::protocol::holomorph::*;
use shared::{self, session};
use server;
use rand::{self, Rng};

impl session::SessionImpl for SessionImpl {
    type Chunk = Chunk;

    fn new(token: Token, chunk: &Chunk) -> Self {
        let salt: String = rand::thread_rng().gen_ascii_chars().take(32).collect();

        let buf = HelloMessage {
            salt: salt.clone(),
        }.as_packet().unwrap();

        send!(chunk, Msg::Write(token, buf));

        SessionImpl {
            server_id: None,
            salt: salt,
            ip: String::new(),
            port: 0,
        }
    }

    fn get_handler(id: u16)
        -> (fn(&mut Session, &Chunk, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            2 => handle_identification,
            3 => handle_state,
            _ => SessionImpl::unhandled,
        }
    }

    fn close(_: Session, _: &Chunk) { }
}

pub fn handle_identification(self_: &mut Session, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
    -> io::Result<()> {

    if self_.server_id.is_some() {
        return Ok(());
    }

    let msg = try!(IdentificationMessage::deserialize(&mut data));

    let gs = match chunk.server.game_servers.get(&msg.id) {
        Some(gs) => gs,
        None => {
            send!(chunk, Msg::Close(self_.token));
            return Ok(());
        }
    };

    if shared::compute_md5(&(shared::compute_md5(&gs.key()) + &self_.salt)) != msg.key {
        send!(chunk, Msg::Close(self_.token));
        return Ok(());
    }

    self_.ip = msg.ip.clone();
    self_.port = msg.port;

    server::register_game_server(&chunk.server.handler, self_.token, msg.id, msg.state,
        msg.ip, msg.port,
        |session, chunk, id| identification_success(session, chunk, id));

    Ok(())
}

fn identification_success(self_: &mut Session, chunk: &Chunk, server_id: Option<i16>) {
    if server_id.is_none() {
        send!(chunk, Msg::Close(self_.token));
        return ();
    }

    self_.server_id = server_id;
}

pub fn handle_state(self_: &mut Session, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
    -> io::Result<()> {

    let server_id = *match self_.server_id.as_ref() {
        Some(server_id) => server_id,
        None => return Ok(())
    };

    let msg = try!(StateMessage::deserialize(&mut data));

    server::update_game_server(&chunk.server.handler, server_id, msg.state,
        self_.ip.clone(), self_.port);

    Ok(())
}
