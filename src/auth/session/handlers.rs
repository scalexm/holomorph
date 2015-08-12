use shared::net::{Token, Msg};
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::connection::*;
use shared::protocol::security::*;
use session::{AccountData, Session};
use session::chunk::Chunk;
use shared::database;
use postgres::{Connection, Result};
use crypto::digest::Digest;
use crypto::md5::Md5;
use server;

enum AuthResult {
    Failure(i8, f64),
    Success(AccountData),
}

impl Session {
    pub fn handle_identification(&mut self, chunk: &Chunk, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        let buf = RawDataMessage {
            content: VarIntVec(chunk.server.patch[0..].to_vec()),
        }.as_packet().unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
        Ok(())
    }

    fn authenticate(conn: &mut Connection, account: String,
        mut password: String) -> AuthResult {

        let stmt = match conn
            .prepare_cached("SELECT * FROM accounts WHERE account = $1") {

            Ok(stmt) => stmt,
            Err(err) => {
                error!("prepare_cached failed: {:?}", err);
                return AuthResult
                    ::Failure(identification_failure_reason::UNKNOWN_AUTH_ERROR, 0.);
            }
        };

        let rows = match stmt.query(&[&account]) {
            Ok(stmt) => stmt,
            Err(err) => {
                error!("query failed: {:?}", err);
                return AuthResult
                    ::Failure(identification_failure_reason::UNKNOWN_AUTH_ERROR, 0.);
            }
        };

        if rows.len() == 0 {
            return AuthResult
                ::Failure(identification_failure_reason::WRONG_CREDENTIALS, 0.);
        }

        let row = rows.get(0);
        let db_password: String = row.get("password");
        let salt: String = row.get("salt");
        let mut md5 = Md5::new();
        md5.input_str(&password);
        password = md5.result_str();
        md5 = Md5::new();
        md5.input_str(&(password + &salt));
        if md5.result_str() != db_password {
            return AuthResult
                ::Failure(identification_failure_reason::WRONG_CREDENTIALS, 0.);
        }

        let level: i16 = row.get("level");
        AuthResult::Success(AccountData {
            id: row.get("id"),
            account: row.get("account"),
            nickname: row.get("nickname"),
            secret_question: row.get("secret_question"),
            level: level as i8,
        })
    }

    fn identification_success(&mut self, chunk: &Chunk, data: AccountData) {

        let mut buf = Vec::new();
        IdentificationSuccessMessage {
            has_rights: Flag(data.level > 0),
            was_already_connected: Flag(false),
            login: (&data.account).to_string(),
            nickname: (&data.nickname).to_string(),
            account_id: data.id,
            community_id: 0,
            secret_question: (&data.secret_question).to_string(),
            account_creation: 0.,
            subscription_elapsed_duration: 0.,
            subscription_end_date: 0.,
        }.as_packet_with_buf(&mut buf).unwrap();

        self.account = Some(data);

        let mut gs = Vec::new();
        for server in &*chunk.server.game_servers {
            gs.push(GameServerInformations {
                id: VarUShort(server.1.id),
                status: 3,
                completion: 0,
                is_selectable: true,
                characters_count: 2,
                date: 0.,
            });
        }
        ServersListMessage {
            servers: gs,
            already_connected_to_server_id: VarUShort(0),
            can_create_new_character: true,
        }.as_packet_with_buf(&mut buf).unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }

    pub fn handle_clear_identification(&mut self, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.account.is_some() {
            return Ok(());
        }

        let msg = try!(ClearIdentificationMessage::deserialize(&mut data));

        let io_loop = chunk.server.io_loop.clone();
        let handler = chunk.server.handler.clone();
        let token = self.token;
        let _ = database::execute(&chunk.server.db, move |conn| {
            match Session::authenticate(conn, msg.username, msg.password) {
                AuthResult::Failure(reason, ban_end) => {
                    let buf = match reason {
                        identification_failure_reason::BANNED =>
                            IdentificationFailedBannedMessage {
                                base: IdentificationFailedMessage { reason: reason, },
                                ban_end_date: ban_end
                            }.as_packet().unwrap(),

                        _ => IdentificationFailedMessage { reason: reason, }
                            .as_packet()
                            .unwrap(),
                    };
                    let _ = io_loop.send(Msg::WriteAndClose(token, buf));
                }

                AuthResult::Success(data) => {
                    server::session_callback(&handler, token, move |session, chunk| {
                        Session::identification_success(session, chunk, data)
                    });
                }
            }
        });

        Ok(())
    }
}
