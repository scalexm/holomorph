use shared::net::{Token, Msg};
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::connection::*;
use shared::protocol::security::*;
use session::{AccountData, Session};
use session::chunk::Chunk;
use postgres::{self, Connection};
use crypto::digest::Digest;
use crypto::md5::Md5;
use server;
use shared::pool;
use time;
use std::collections::HashMap;

enum AuthError {
    SqlError(postgres::error::Error),
    Reason(i8),
    Banned(i64),
}

impl From<postgres::error::Error> for AuthError {
    fn from(err: postgres::error::Error) -> AuthError {
        AuthError::SqlError(err)
    }
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
        mut password: String) -> Result<AccountData, AuthError> {

        let trans = try!(conn.transaction());
        let stmt = try!(trans.prepare_cached("SELECT * FROM accounts WHERE account = $1"));
        let rows = try!(stmt.query(&[&account]));

        if rows.len() == 0 {
            return Err(AuthError::Reason(identification_failure_reason::WRONG_CREDENTIALS));
        }

        let row = rows.get(0);
        let ban_end: i64 = row.get("ban_end");

        if ban_end < 0 {
            return Err(AuthError::Reason(identification_failure_reason::BANNED));
        }

        if ban_end > time::get_time().sec {
            return Err(AuthError::Banned(ban_end));
        }

        let db_password: String = row.get("password");
        let salt: String = row.get("salt");
        let mut md5 = Md5::new();
        md5.input_str(&password);
        password = md5.result_str();
        md5 = Md5::new();
        md5.input_str(&(password + &salt));
        if md5.result_str() != db_password {
            return Err(AuthError::Reason(identification_failure_reason::WRONG_CREDENTIALS));
        }

        let id: i32 = row.get("id");
        let mut map = HashMap::new();
        let stmt = try!(trans
            .prepare_cached("SELECT server_id FROM character_counts WHERE account_id = $1"));
        let rows = try!(stmt.query(&[&id]));
        for row in rows {
            let id: i16 = row.get("server_id");
            let val = map.get(&id).unwrap_or(&0) + 1;
            let _ = map.insert(id, val);
        }

        try!(trans.commit());

        let level: i16 = row.get("level");
        Ok(AccountData {
            id: id,
            account: row.get("account"),
            nickname: row.get("nickname"),
            secret_question: row.get("secret_question"),
            level: level as i8,
            subscription_end: row.get("subscription_end"),
            subscription_elapsed: row.get("subscription_elapsed"),
            creation: row.get("creation"),
            character_counts: map,
            already_logged: row.get("logged"),
        })
    }

    fn identification_success(&mut self, chunk: &Chunk, data: AccountData,
        already_logged: bool) {
        debug!("{}", already_logged);

        let mut buf = Vec::new();
        IdentificationSuccessMessage {
            has_rights: Flag(data.level > 0),
            was_already_connected: Flag(already_logged || data.already_logged != 0),
            login: (&data.account).to_string(),
            nickname: (&data.nickname).to_string(),
            account_id: data.id,
            community_id: 0,
            secret_question: (&data.secret_question).to_string(),
            account_creation: (data.creation * 1000) as f64,
            subscription_elapsed_duration: (data.subscription_elapsed * 1000) as f64,
            subscription_end_date: (data.subscription_end * 1000) as f64,
        }.as_packet_with_buf(&mut buf).unwrap();

        let mut gs = Vec::new();
        for server in &*chunk.server.game_servers {
            gs.push(GameServerInformations {
                id: VarUShort(server.1.id as u16),
                status: 3,
                completion: 0,
                is_selectable: true,
                characters_count: *data
                    .character_counts
                    .get(&server.1.id)
                    .unwrap_or(&0) as i8,
                date: 0.,
            });
        }

        ServersListMessage {
            servers: gs,
            already_connected_to_server_id: VarUShort(data.already_logged as u16),
            can_create_new_character: true,
        }.as_packet_with_buf(&mut buf).unwrap();

        self.account = Some(data);
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
        let _ = pool::execute(&chunk.server.db, move |conn| {
            match Session::authenticate(conn, msg.username, msg.password) {
                Err(err) => {
                    let buf = match err {
                        AuthError::Banned(ban_end) =>
                            IdentificationFailedBannedMessage {
                                base: IdentificationFailedMessage {
                                    reason: identification_failure_reason::BANNED,
                                },
                                ban_end_date: (ban_end * 1000) as f64,
                            }.as_packet().unwrap(),

                        AuthError::Reason(reason) =>
                            IdentificationFailedMessage { reason: reason, }
                                .as_packet()
                                .unwrap(),

                        AuthError::SqlError(err) => {
                            error!("authenticate sql error: {}", err);
                            IdentificationFailedMessage {
                                reason: identification_failure_reason::UNKNOWN_AUTH_ERROR,
                            }.as_packet().unwrap()
                        }
                    };
                    let _ = io_loop.send(Msg::WriteAndClose(token, buf));
                }

                Ok(data) => {
                    let id = data.id;
                    server::identification_success(&handler, token, id,
                        move |session, chunk, already| {

                        Session::identification_success(session, chunk, data, already)
                    });
                }
            }
        });

        Ok(())
    }
}
