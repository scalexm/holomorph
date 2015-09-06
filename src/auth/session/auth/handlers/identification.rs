use shared::net::Msg;
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::messages::connection::*;
use shared::protocol::messages::security::*;
use shared::protocol::messages::queues::*;
use shared::protocol::enums::{server_status, identification_failure_reason};
use session::auth::{handlers, AccountData, Session, QueueState};
use session::auth::chunk::Chunk;
use postgres::{self, Connection};
use server;
use shared::{self, database};
use time;
use std::collections::HashMap;
use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};

pub static QUEUE_SIZE: AtomicIsize = ATOMIC_ISIZE_INIT;
pub static QUEUE_COUNTER: AtomicIsize = ATOMIC_ISIZE_INIT;

enum Error {
    SqlError(postgres::error::Error),
    Reason(i8),
    Banned(i64),
}

impl From<postgres::error::Error> for Error {
    fn from(err: postgres::error::Error) -> Error {
        Error::SqlError(err)
    }
}

fn authenticate(conn: &mut Connection, account: String, password: String,
    addr: String) -> Result<AccountData, Error> {

    let trans = try!(conn.transaction());

    let stmt = try!(trans.prepare_cached("SELECT 1 FROM ip_bans WHERE ip
        = $1"));

    if try!(stmt.query(&[&addr])).len() != 0 {
        return Err(Error::Reason(identification_failure_reason::WRONG_CREDENTIALS));
    }

    let stmt = try!(trans.prepare_cached("SELECT * FROM accounts WHERE account = $1"));
    let rows = try!(stmt.query(&[&account]));

    if rows.len() == 0 {
        return Err(Error::Reason(identification_failure_reason::WRONG_CREDENTIALS));
    }

    let row = rows.get(0);
    let ban_end: i64 = try!(row.get_opt("ban_end"));

    if ban_end < 0 {
        return Err(Error::Reason(identification_failure_reason::BANNED));
    }

    if ban_end > time::get_time().sec {
        return Err(Error::Banned(ban_end));
    }

    let db_password: String = try!(row.get_opt("password"));
    let salt: String = try!(row.get_opt("salt"));
    if shared::compute_md5(&(shared::compute_md5(&password) + &salt)) != db_password {
        return Err(Error::Reason(identification_failure_reason::WRONG_CREDENTIALS));
    }

    let id: i32 = try!(row.get_opt("id"));
    let mut character_counts = HashMap::new();

    let stmt = try!(trans
        .prepare_cached("SELECT server_id FROM character_counts WHERE account_id = $1"));
    let rows = try!(stmt.query(&[&id]));
    for row in rows {
        let id: i16 = try!(row.get_opt("server_id"));
        *character_counts.entry(id).or_insert(0) += 1;
    }

    try!(trans.commit());

    let level: i16 = try!(row.get_opt("level"));
    Ok(AccountData {
        id: id,
        account: try!(row.get_opt("account")),
        nickname: try!(row.get_opt("nickname")),
        secret_question: try!(row.get_opt("secret_question")),
        level: level as i8,
        subscription_end: try!(row.get_opt("subscription_end")),
        subscription_elapsed: 0,
        creation_date: try!(row.get_opt("creation_date")),
        character_counts: character_counts,
        already_logged: try!(row.get_opt("logged")),
        last_server: try!(row.get_opt("last_server")),
    })
}

fn identification_success(self_: &mut Session, chunk: &Chunk, data: AccountData,
    already_logged: bool, auto_connect: bool) {

    self_.queue_state = QueueState::None;
    self_.account = Some(data);
    let data = self_.account.as_ref().unwrap();
    let subscriber = data.is_subscriber();

    let mut buf = LoginQueueStatusMessage {
        position: 0,
        total: 0,
    }.as_packet().unwrap();

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

    send!(chunk, Msg::Write(self_.token, buf));

    if auto_connect {
        if handlers::selection::select_server(self_, chunk, data.last_server)
            .ok()
            .is_some() {

            return ();
        }
    }

    let servers = chunk.server.game_servers.values().filter_map(|server| {
        if server.min_level() > data.level {
            return None;
        }

        let status = chunk.game_status
            .get(&server.id())
            .map(|status| status.0)
            .unwrap_or(server_status::OFFLINE);

        Some(handlers::get_server_informations(self_, server, status))
    }).collect();

    let buf = ServersListMessage {
        servers: servers,
        already_connected_to_server_id: VarShort(data.already_logged),
        can_create_new_character: true,
    }.as_packet().unwrap();

    send!(chunk, Msg::Write(self_.token, buf));
}

pub fn handle_identification(self_: &mut Session, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
    -> io::Result<()> {

    use std::io::Read;
    use shared::io::ReadExt;

    if self_.account.is_some() || !self_.queue_state.is_none() {
        return Ok(());
    }

    if !self_.custom_identification {
        self_.custom_identification = true;
        let buf = RawDataMessage {
            content: VarIntVec(chunk.server.patch[0..].to_vec()),
        }.as_packet().unwrap();

        send!(chunk, Msg::Write(self_.token, buf));
        return Ok(());
    }

    let msg = try!(IdentificationMessage::deserialize(&mut data));

    let mut credentials = Cursor::new(msg.credentials.0);
    let username = try!(credentials.read_string());
    let password = try!(credentials.read_string());
    try!(credentials.read_to_end(&mut self_.aes_key));
    let auto_connect = msg.autoconnect.0;

    let io_loop = chunk.server.io_loop.clone();
    let handler = chunk.server.handler.clone();
    let token = self_.token;
    let addr = self_.address.clone();

    self_.queue_state = QueueState::Some(QUEUE_SIZE.fetch_add(1, Ordering::Relaxed) + 1,
        QUEUE_COUNTER.load(Ordering::Relaxed));

    database::execute(&chunk.server.db, move |conn| {
        match authenticate(conn, username, password, addr) {
            Err(err) => {
                let buf = match err {
                    Error::Banned(ban_end) =>
                        IdentificationFailedBannedMessage {
                            base: IdentificationFailedMessage {
                                reason: identification_failure_reason::BANNED,
                            },
                            ban_end_date: (ban_end * 1000) as f64,
                        }.as_packet().unwrap(),

                    Error::Reason(reason) =>
                        IdentificationFailedMessage { reason: reason, }
                            .as_packet()
                            .unwrap(),

                    Error::SqlError(err) => {
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
                        identification_success(session, chunk, data, already, auto_connect)
                    });
            }
        }

        let _ = QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed);
        let _ = QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    });

    Ok(())
}
