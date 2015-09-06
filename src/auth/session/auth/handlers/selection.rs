use shared::net::Msg;
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::messages::connection::*;
use shared::protocol::enums::{server_status, server_connection_error};
use session::auth::Session;
use session::auth::chunk::Chunk;
use postgres::{self, Connection};
use shared::database;
use rand::{self, Rng};
use crypto::aes;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer, BufferResult, WriteBuffer, ReadBuffer};
use crypto::symmetriccipher::Encryptor;
use crypto::blockmodes::NoPadding;

pub struct Error(i8, i8);

fn update_ticket(conn: &mut Connection, id: i32, ticket: String)
    -> Result<(), postgres::error::Error> {

    let stmt = try!(conn.prepare_cached("UPDATE accounts SET ticket = $1 WHERE id = $2"));
    let _ = try!(stmt.execute(&[&ticket, &id]));

    Ok(())
}

pub fn select_server(self_: &Session, chunk: &Chunk, server_id: i16)
    -> Result<(), Error> {

    let ref account = self_.account.as_ref().unwrap();

    let status = match chunk.game_status.get(&server_id) {
        Some(status) => status,
        None => return Err(Error(server_connection_error::NO_REASON, server_status::OFFLINE)),
    };

    let server = chunk.server.game_servers.get(&server_id).unwrap();

    if status.0 != server_status::ONLINE && status.0 != server_status::FULL {
        return Err(Error(server_connection_error::DUE_TO_STATUS, status.0));
    }

    if status.0 == server_status::FULL && !account.is_subscriber() {
        return Err(Error(server_connection_error::SUBSCRIBERS_ONLY, status.0));
    }

    if server.min_level() > account.level {
        return Err(Error(server_connection_error::ACCOUNT_RESTRICTED, status.0));
    }

    let ticket: String = rand::thread_rng().gen_ascii_chars().take(32).collect();
    let mut result = Vec::new();

    {
        let mut cbc = aes::cbc_encryptor(aes::KeySize::KeySize256, &self_.aes_key[0..32],
            &self_.aes_key[0..16], NoPadding);

        let mut output = [0; 32];
        let mut read_buffer = RefReadBuffer::new(&ticket.as_bytes());
        let mut write_buffer = RefWriteBuffer::new(&mut output);

        loop {
            let res = match cbc.encrypt(&mut read_buffer, &mut write_buffer, true) {
                Ok(res) => res,
                Err(err) => {
                    // debug only because it means the key is malformed
                    debug!("select_server cipher error: {:?}", err);
                    return Err(Error(server_connection_error::NO_REASON, status.0));
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
        server_id: VarShort(server_id),
        address: status.1.clone(),
        port: status.2,
        can_create_new_character: true,
        ticket: VarIntVec(result),
    }.as_packet().unwrap();

    send!(chunk, Msg::Write(self_.token, buf));

    let io_loop = chunk.server.io_loop.clone();
    let id = account.id;
    let tok = self_.token;
    let status = status.0;

    database::execute(&chunk.server.db, move |conn| {
        match update_ticket(conn, id, ticket) {
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
    });

    Ok(())
}

pub fn handle_server_selection(self_: &mut Session, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
    -> io::Result<()> {

    if self_.account.is_none() {
        return Ok(());
    }

    let msg = try!(ServerSelectionMessage::deserialize(&mut data));
    let server_id = msg.server_id.0 as i16;

    if let Err(err) = select_server(self_, chunk, server_id) {
        let buf = SelectedServerRefusedMessage {
            server_id: VarShort(server_id),
            error: err.0,
            server_status: err.1,
        }.as_packet().unwrap();
        send!(chunk, Msg::Write(self_.token, buf));
    }

    Ok(())
}
