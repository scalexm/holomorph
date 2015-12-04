use std::io::Result;
use session::game::Session;
use session::game::chunk::Ref;
use protocol::Protocol;
use protocol::holomorph::*;
use shared::crypt;
use server::{self, SERVER};

impl Session {
    fn identification_success(&mut self, server_id: i16) {
        self.server_id = Some(server_id);
    }
}

#[register_handlers]
impl Session {
    pub fn handle_identification<'a>(&mut self, _: Ref<'a>, msg: IdentificationMessage)
                                     -> Result<()> {
        if self.server_id.is_some() {
            return Ok(());
        }

        let md5_key = match SERVER.with(|s| {
            s.game_servers
             .get(&msg.id)
             .map(|gs| crypt::md5(&gs.key()))
         }) {
            Some(key) => key,
            None => {
                close!(SERVER, self.base.token);
                return Ok(());
            }
        };

        if crypt::md5(&(md5_key + &self.salt)) != msg.key {
            close!(SERVER, self.base.token);
            return Ok(());
        }

        self.ip = msg.ip.clone();
        self.port = msg.port;

        SERVER.with(move |s| {
            server::register_game_server(
                &s.server,
                self.base.token,
                msg.id,
                msg.state,
                msg.ip,
                msg.port,
                |session, id| session.identification_success(id)
            );
        });

        Ok(())
    }

    pub fn handle_state<'a>(&mut self, _: Ref<'a>, msg: StateMessage) -> Result<()> {
        let server_id = *match self.server_id.as_ref() {
            Some(server_id) => server_id,
            None => return Ok(())
        };

        SERVER.with(move |s| {
            server::update_game_server(&s.server, server_id, msg.state,
                self.ip.clone(), self.port);
        });

        Ok(())
    }
}
