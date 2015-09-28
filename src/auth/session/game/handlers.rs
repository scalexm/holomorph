use std::io::{self, Cursor};
use super::Session;
use super::chunk::{Ref, ChunkImpl};
use shared::protocol::*;
use shared::protocol::holomorph::*;
use shared;
use shared::session::{self, SessionBase};
use server::{self, SERVER};
use rand::{self, Rng};

impl session::Session<ChunkImpl> for Session {
    fn new(base: SessionBase) -> Self {
        let salt: String = rand::thread_rng().gen_ascii_chars().take(32).collect();

        let buf = HelloMessage {
            salt: salt.clone(),
        }.as_packet().unwrap();

        write!(SERVER, base.token, buf);

        Session {
            base: base,
            server_id: None,
            salt: salt,
            ip: String::new(),
            port: 0,
        }
    }

    fn get_handler<'a>(id: u16)
        -> (fn(&mut Session, Ref<'a>, Cursor<Vec<u8>>) -> io::Result<()>) {

        match id {
            2 => Session::handle_identification,
            3 => Session::handle_state,
            _ => Session::unhandled,
        }
    }

    fn close<'a>(self, _: Ref<'a>) { }
}

impl Session {
    pub fn handle_identification<'a>(&mut self, _: Ref<'a>, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.server_id.is_some() {
            return Ok(());
        }

        let msg = try!(IdentificationMessage::deserialize(&mut data));

        let md5_key = match SERVER.with(|s| s.game_servers
            .get(&msg.id)
            .map(|gs| shared::compute_md5(&gs.key()))) {

            Some(key) => key,
            None => {
                close!(SERVER, self.base.token);
                return Ok(());
            }
        };

        if shared::compute_md5(&(md5_key + &self.salt)) != msg.key {
            close!(SERVER, self.base.token);
            return Ok(());
        }

        self.ip = msg.ip.clone();
        self.port = msg.port;

        SERVER.with(move |s| {
            server::register_game_server(&s.server, self.base.token, msg.id,
                msg.state, msg.ip, msg.port,
                |session, id| session.identification_success(id));
        });

        Ok(())
    }

    fn identification_success(&mut self, server_id: i16) {
        self.server_id = Some(server_id);
    }

    pub fn handle_state<'a>(&mut self, _: Ref<'a>, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        let server_id = *match self.server_id.as_ref() {
            Some(server_id) => server_id,
            None => return Ok(())
        };

        let msg = try!(StateMessage::deserialize(&mut data));

        SERVER.with(move |s| {
            server::update_game_server(&s.server, server_id, msg.state,
                self.ip.clone(), self.port);
        });

        Ok(())
    }
}
