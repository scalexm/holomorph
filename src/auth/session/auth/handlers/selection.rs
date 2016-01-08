use std::io;
use protocol::{Protocol, VarIntVec, VarShort};
use protocol::messages::connection::*;
use protocol::enums::{server_status, server_connection_error};
use session::auth::Session;
use session::auth::chunk::{Ref, ChunkImpl};
use diesel::*;
use shared::{crypto, database};
use rand::{self, Rng};
use server::SERVER;

pub struct Error(i8, i8);

fn update_ticket(conn: &Connection, id: i32, ticket: &str) -> QueryResult<()> {
    use diesel::query_builder::update;
    use shared::database::schema::accounts;

    update(
        accounts::table.filter(accounts::id.eq(&id))
    ).set(accounts::ticket.eq(ticket))
     .execute(conn).map(|_| ())
}

impl Session {
    pub fn select_server(&self, chunk: &ChunkImpl, server_id: i16)
                         -> Result<(), Error> {
        let account = self.account.as_ref().unwrap();

        let status = match chunk.game_status.get(&server_id) {
            Some(status) => status,
            None => return Err(Error(server_connection_error::NO_REASON, server_status::OFFLINE)),
        };

        if status.0 != server_status::ONLINE && status.0 != server_status::FULL {
            return Err(Error(server_connection_error::DUE_TO_STATUS, status.0));
        }

        if status.0 == server_status::FULL && !account.is_subscriber() {
            return Err(Error(server_connection_error::SUBSCRIBERS_ONLY, status.0));
        }

        let min_level = SERVER.with(|s| s.game_servers.get(&server_id).unwrap().min_level());

        if min_level > account.level {
            return Err(Error(server_connection_error::ACCOUNT_RESTRICTED, status.0));
        }

        let ticket: String = rand::thread_rng().gen_ascii_chars().take(32).collect();
        log_info!(
            self,
            "server selection: server_id = {}, ticket = {}",
            server_id,
            ticket
        );

        let result = match crypto::aes_256(&self.aes_key[0..32],
                                           &self.aes_key[0..16],
                                           ticket.as_bytes()) {
            Ok(result) => result,
            Err(err) => {
                log_err!(self, "ticket encryption failed: {:?}", err);
                return Err(Error(server_connection_error::NO_REASON, status.0));
            }
        };

        let buf = SelectedServerDataMessage {
            server_id: VarShort(server_id),
            address: status.1.clone(),
            port: status.2,
            can_create_new_character: true,
            ticket: VarIntVec(result),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);

        let id = account.id;
        let tok = self.base.token;
        let status = status.0;
        let io_loop = SERVER.with(|s| s.io_loop.clone());

        SERVER.with(|s| database::execute(&s.db, move |conn| {
            use shared::net::Msg;
            match update_ticket(conn, id, &ticket) {
                Ok(()) => {
                    // the client expects this socket shutdown
                    let _ = io_loop.send(Msg::Close(tok));
                }

                Err(err) => {
                    error!("update_ticket sql error: {}", err);
                    let buf = SelectedServerRefusedMessage {
                        server_id: VarShort(server_id),
                        error: server_connection_error::NO_REASON,
                        server_status: status,
                    }.as_packet().unwrap();
                    let _ = io_loop.send(Msg::Write(tok, buf));
                }
            }
        }));

        Ok(())
    }
}

#[register_handlers]
impl Session {
    pub fn handle_server_selection<'a>(&mut self, chunk: Ref<'a>, msg: ServerSelectionMessage)
                                       -> io::Result<()> {
        if self.account.is_none() {
            return Ok(());
        }

        let server_id = msg.server_id.0 as i16;

        if let Err(err) = self.select_server(&*chunk, server_id) {
            let buf = SelectedServerRefusedMessage {
                server_id: VarShort(server_id),
                error: err.0,
                server_status: err.1,
            }.as_packet().unwrap();
            write!(SERVER, self.base.token, buf);
        }

        Ok(())
    }
}
