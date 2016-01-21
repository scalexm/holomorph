use session::auth::Session;
use session::auth::chunk::Ref;
use protocol::*;
use protocol::holomorph::*;
use shared::crypto;
use std::io::Result;
use server::{self, SERVER};

#[register_handlers]
impl Session {
    pub fn handle_hello<'a>(&mut self, _: Ref<'a>, msg: HelloMessage) -> Result<()> {
        let md5_key = SERVER.with(|s| crypto::md5(&s.cnf.server_key));

        let buf = IdentificationMessage {
            id: SERVER.with(|s| s.cnf.server_id),
            key: crypto::md5(&(md5_key + &msg.salt)),
            state: 3,
            ip: SERVER.with(|s| s.cnf.bind_ip.clone()),
            port: SERVER.with(|s| s.cnf.bind_port),
        }.unwrap();

        write!(SERVER, self.base.token, buf);
        Ok(())
    }

    pub fn handle_disconnect_player<'a>(&mut self, _: Ref<'a>, msg: DisconnectPlayerMessage)
                                        -> Result<()> {
        SERVER.with(|s| server::disconnect_player(&s.server, msg.id));
        Ok(())
    }
}
