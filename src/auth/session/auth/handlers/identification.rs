use shared::net::Msg;
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::messages::connection::*;
use shared::protocol::messages::security::*;
use shared::protocol::messages::queues::*;
use shared::protocol::enums::{server_status, identification_failure_reason};
use session::auth::{AccountData, Session, Chunk, QueueState};
use postgres::{self, Connection};
use server;
use shared::{self, database};
use time;
use std::collections::HashMap;
use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};

pub static QUEUE_SIZE: AtomicIsize = ATOMIC_ISIZE_INIT;
pub static QUEUE_COUNTER: AtomicIsize = ATOMIC_ISIZE_INIT;

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
    fn authenticate(conn: &mut Connection, account: String, password: String,
        addr: String) -> Result<AccountData, AuthError> {

        let trans = try!(conn.transaction());

        let stmt = try!(trans.prepare_cached("SELECT 1 FROM ip_bans WHERE ip
            = $1"));

        if try!(stmt.query(&[&addr])).len() != 0 {
            return Err(AuthError::Reason(identification_failure_reason::WRONG_CREDENTIALS));
        }

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
        if shared::compute_md5(&(shared::compute_md5(&password) + &salt)) != db_password {
            return Err(AuthError::Reason(identification_failure_reason::WRONG_CREDENTIALS));
        }

        let id: i32 = row.get("id");
        let mut character_counts = HashMap::new();

        let stmt = try!(trans
            .prepare_cached("SELECT server_id FROM character_counts WHERE account_id = $1"));
        let rows = try!(stmt.query(&[&id]));
        for row in rows {
            let id: i16 = row.get("server_id");
            let val = character_counts.get(&id).unwrap_or(&0) + 1;
            let _ = character_counts.insert(id, val);
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
            subscription_elapsed: 0,
            creation_date: row.get("creation_date"),
            character_counts: character_counts,
            already_logged: row.get("logged"),
            last_server: row.get("last_server"),
        })
    }

    fn identification_success(&mut self, chunk: &Chunk, data: AccountData,
        already_logged: bool, auto_connect: bool) {

        let mut buf = Vec::new();
        let subscriber = data.is_subscriber();

        self.queue_state = QueueState::None;

        LoginQueueStatusMessage {
            position: 0,
            total: 0,
        }.as_packet_with_buf(&mut buf).unwrap();

        self.account = Some(data);
        let data = self.account.as_ref().unwrap();

        IdentificationSuccessMessage {
            has_rights: Flag(data.level > 0),
            was_already_connected: Flag(already_logged || data.already_logged != 0),
            login: data.account.clone(),
            nickname: data.nickname.clone(),
            account_id: data.id,
            community_id: 0,
            secret_question: data.secret_question.clone(),
            account_creation: (data.creation_date * 1000) as f64,
            subscription_elapsed_duration: (data.subscription_elapsed * 1000) as f64,
            subscription_end_date: match subscriber {
                true => data.subscription_end * 1000,
                false => 0,
            } as f64,
        }.as_packet_with_buf(&mut buf).unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));

        if auto_connect {
            if self.select_server(chunk, data.last_server).ok().is_some() {
                return ();
            }
        }

        let mut gs = Vec::new();
        for server in &*chunk.server.game_servers {
            if server.1.min_level > data.level {
                continue;
            }

            let status = chunk.game_status
                .get(&server.1.id)
                .map(|status| status.0)
                .unwrap_or(server_status::OFFLINE);

            gs.push(self.get_server_informations(&server.1, status));
        }

        let buf = ServersListMessage {
            servers: gs,
            already_connected_to_server_id: VarShort(data.already_logged),
            can_create_new_character: true,
        }.as_packet().unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }

    pub fn handle_identification(&mut self, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        use std::io::Read;
        use shared::io::ReadExt;

        if self.account.is_some() || !self.queue_state.is_none() {
            return Ok(());
        }

        if !self.custom_identification {
            self.custom_identification = true;
            let buf = RawDataMessage {
                content: VarIntVec(chunk.server.patch[0..].to_vec()),
            }.as_packet().unwrap();

            let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
            return Ok(());
        }

        let msg = try!(IdentificationMessage::deserialize(&mut data));

        let mut credentials = Cursor::new(msg.credentials.0);
        let username = try!(credentials.read_string());
        let password = try!(credentials.read_string());
        try!(credentials.read_to_end(&mut self.aes_key));
        let auto_connect = msg.autoconnect.0;

        let io_loop = chunk.server.io_loop.clone();
        let handler = chunk.server.handler.clone();
        let token = self.token;
        let addr = self.address.clone();

        self.queue_state = QueueState::Some(QUEUE_SIZE.fetch_add(1, Ordering::Relaxed) + 1,
            QUEUE_COUNTER.load(Ordering::Relaxed));

        database::execute(&chunk.server.db, move |conn| {
            match Session::authenticate(conn, username, password, addr) {
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
                    server::identification_success(&handler, token, id, data.already_logged,
                        move |session, chunk, already| {
                            Session::identification_success(session, chunk, data, already,
                                auto_connect)
                    });
                }
            }

            let _ = QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed);
            let _ = QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
        });

        Ok(())
    }
}
