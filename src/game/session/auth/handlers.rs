use super::Session;
use super::chunk::{ChunkImpl, Ref};
use protocol::*;
use protocol::holomorph::*;
use shared::{self, crypt};
use std::io::{Result, Cursor};
use server::{self, SERVER};

impl shared::session::Session<ChunkImpl> for Session {
    fn new(base: shared::session::SessionBase) -> Self {
        Session {
            base: base,
        }
    }

    fn get_handler<'a>(id: u16) -> (fn(&mut Session, Ref<'a>, Cursor<Vec<u8>>) -> Result<()>) {
        match id {
            1 => Session::handle_hello,
            4 => Session::handle_disconnect_player,
            _ => Session::unhandled,
        }
    }

    fn close<'a>(self, _: Ref<'a>) {
        error!("FATAL ERROR: lost connection with auth server");
        SERVER.with(|s| s.shutdown());
    }
}

impl Session {
    fn handle_hello<'a>(&mut self, _: Ref<'a>, mut data: Cursor<Vec<u8>>) -> Result<()> {
        let msg = try!(HelloMessage::deserialize(&mut data));
        let md5_key = SERVER.with(|s| crypt::md5(&s.cnf.server_key));

        let buf = IdentificationMessage {
            id: SERVER.with(|s| s.cnf.server_id),
            key: crypt::md5(&(md5_key + &msg.salt)),
            state: 3,
            ip: SERVER.with(|s| s.cnf.bind_ip.clone()),
            port: SERVER.with(|s| s.cnf.bind_port),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);
        Ok(())
    }

    pub fn handle_disconnect_player<'a>(&mut self, _: Ref<'a>, mut data: Cursor<Vec<u8>>)
                                        -> Result<()> {

        let msg = try!(DisconnectPlayerMessage::deserialize(&mut data));
        SERVER.with(|s| server::disconnect_player(&s.server, msg.id));

        Ok(())
    }
}
