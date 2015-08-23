use shared::net::Msg;
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::connection::*;
use session::auth::{Session, Chunk};
use postgres::{self, Connection};
use shared::database;
use rand::{self, Rng};
use crypto::aes;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer, BufferResult, WriteBuffer, ReadBuffer};
use crypto::symmetriccipher::Encryptor;
use crypto::blockmodes::PkcsPadding;

struct ServerSelectionError(i8, i8);

impl Session {
    fn update_ticket(conn: &mut Connection, id: i32, ticket: String)
        -> Result<(), postgres::error::Error> {

        let stmt = try!(conn.prepare_cached("UPDATE accounts SET ticket = $1 WHERE id = $2"));
        let _ = try!(stmt.execute(&[&ticket, &id]));

        Ok(())
    }

    pub fn select_server(&self, chunk: &Chunk, server_id: i16)
        -> Result<(), ServerSelectionError> {

        let ref account = self.account.as_ref().unwrap();

        let status = chunk.game_status.get(&server_id);
        if status.is_none() {
            return Err(ServerSelectionError(server_connection_error::NO_REASON,
                server_status::OFFLINE));
        }

        let ref status = status.unwrap();
        let server = chunk.server.game_servers.get(&server_id).unwrap();

        if status.0 != server_status::ONLINE && status.0 != server_status::FULL {
            return Err(ServerSelectionError(server_connection_error::DUE_TO_STATUS,
                status.0));
        }

        if status.0 == server_status::FULL && !account.is_subscriber() {
            return Err(ServerSelectionError(server_connection_error::SUBSCRIBERS_ONLY,
                status.0));
        }

        if server.min_level > account.level {
            return Err(ServerSelectionError(server_connection_error::ACCOUNT_RESTRICTED,
                status.0));
        }

        let ticket: String = rand::thread_rng().gen_ascii_chars().take(10).collect();
        let mut result = Vec::new();

        {
            let mut cbc = aes::cbc_encryptor(aes::KeySize::KeySize256, &self.aes_key[0..32],
                &self.aes_key[0..16], PkcsPadding);

            let mut output = [0; 16];
            let mut read_buffer = RefReadBuffer::new(&ticket.as_bytes());
            let mut write_buffer = RefWriteBuffer::new(&mut output);

            loop {
                let res = match cbc.encrypt(&mut read_buffer, &mut write_buffer, true) {
                    Ok(res) => res,
                    Err(err) => {
                        // debug only because it means the key is malformed
                        debug!("select_server cipher error: {:?}", err);
                        return Err(ServerSelectionError(server_connection_error::NO_REASON,
                            status.0));
                    }
                };

                result.extend(write_buffer
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|&i| i));

                if let BufferResult::BufferUnderflow = res {
                    break;
                }
            }
        }

        let buf = SelectedServerDataMessage {
            server_id: VarUShort(server_id as u16),
            address: status.1.clone(),
            port: status.2,
            can_create_new_character: true,
            ticket: VarIntVec(result),
        }.as_packet().unwrap();
        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));

        let io_loop = chunk.server.io_loop.clone();
        let id = account.id;
        let tok = self.token;
        let status = status.0;

        database::execute(&chunk.server.db, move |conn| {
            match Session::update_ticket(conn, id, ticket) {
                Ok(()) => {
                    // the client expects this socket shutdown
                    let _ = io_loop.send(Msg::Close(tok));
                }

                Err(err) => {
                    error!("update_ticket sql error: {}", err);
                    let buf = SelectedServerRefusedMessage {
                        server_id: VarUShort(server_id as u16),
                        error: server_connection_error::NO_REASON,
                        server_status: status,
                    }.as_packet().unwrap();
                    let _ = io_loop.send(Msg::Write(tok, buf));
                }
            }
        });

        Ok(())
    }

    pub fn handle_server_selection(&mut self, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.account.is_none() {
            return Ok(());
        }

        let msg = try!(ServerSelectionMessage::deserialize(&mut data));
        let server_id = msg.server_id.0 as i16;

        if let Err(err) = self.select_server(chunk, server_id) {
            let buf = SelectedServerRefusedMessage {
                server_id: VarUShort(server_id as u16),
                error: err.0,
                server_status: err.1,
            }.as_packet().unwrap();
            let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
        }

        Ok(())
    }
}
