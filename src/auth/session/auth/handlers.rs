use shared::net::Msg;
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::connection::*;
use shared::protocol::handshake::*;
use shared::protocol::security::*;
use shared::protocol::queues::*;
use super::{AccountData, Session, Chunk};
use postgres::{self, Connection};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::aes;
use crypto::blockmodes::NoPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer, BufferResult, WriteBuffer, ReadBuffer};
use crypto::symmetriccipher::Encryptor;
use crypto::blockmodes::PkcsPadding;
use server;
use shared::database;
use time;
use std::collections::HashMap;
use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};
use server::data::GameServerData;
use rand::{self, Rng};

static QUEUE_SIZE: AtomicIsize = ATOMIC_ISIZE_INIT;
static QUEUE_COUNTER: AtomicIsize = ATOMIC_ISIZE_INIT;

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
    pub fn start(&self, chunk: &Chunk) {
        let mut buf = Vec::new();
        ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet_with_buf(&mut buf).unwrap();

        HelloConnectMessage {
            salt: "salut".to_string(),
            key: VarIntVec((*chunk.server.key).clone()),
        }.as_packet_with_buf(&mut buf).unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }

    pub fn update_queue(&self, chunk: &Chunk) {
        if self.queue_size == -1 {
            return ();
        }

        let mut pos = self.queue_size -
            (QUEUE_COUNTER.load(Ordering::Relaxed) - self.queue_counter);

        if pos < 0 {
            pos = 0;
        }

        let buf = LoginQueueStatusMessage {
            position: pos as u16,
            total: QUEUE_SIZE.load(Ordering::Relaxed) as u16,
        }.as_packet().unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
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
            subscription_elapsed: 0,
            creation: row.get("creation"),
            character_counts: map,
            already_logged: row.get("logged"),
        })
    }

    fn get_server_informations(&self, server: &GameServerData, mut status: i8)
        -> GameServerInformations {

        let data = self.account.as_ref().unwrap();

        if data.is_subscriber() && status == server_status::FULL {
            status = server_status::ONLINE;
        }

        GameServerInformations {
            id: VarUShort(server.id as u16),
            status: status,
            completion: 0,
            is_selectable: status == server_status::ONLINE,
            characters_count: *data
                .character_counts
                .get(&server.id)
                .unwrap_or(&0) as i8,
            date: 0.,
        }
    }

    pub fn update_server_status(&self, chunk: &Chunk, server_id: i16, status: i8) {
        if self.account.is_none() {
            return ();
        }

        let server = chunk.server.game_servers.get(&server_id);
        if server.is_none() {
            return ();
        }

        let server = server.unwrap();

        if server.min_level > self.account.as_ref().unwrap().level {
            return ();
        }

        let buf = ServerStatusUpdateMessage {
            server: self.get_server_informations(&server, status),
        }.as_packet().unwrap();
        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }

    fn identification_success(&mut self, chunk: &Chunk, data: AccountData,
        already_logged: bool) {

        let mut buf = Vec::new();
        let subscriber = data.is_subscriber();

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
            account_creation: (data.creation * 1000) as f64,
            subscription_elapsed_duration: (data.subscription_elapsed * 1000) as f64,
            subscription_end_date: match subscriber {
                true => data.subscription_end * 1000,
                false => 0,
            } as f64,
        }.as_packet_with_buf(&mut buf).unwrap();

        let mut gs = Vec::new();
        for server in &*chunk.server.game_servers {
            if server.1.min_level > data.level {
                continue;
            }

            let status = chunk.game_status
                .get(&server.1.id)
                .map(|status| status.0)
                .unwrap_or(server_status::ONLINE);

            gs.push(self.get_server_informations(&server.1, status));
        }

        ServersListMessage {
            servers: gs,
            already_connected_to_server_id: VarUShort(data.already_logged as u16),
            can_create_new_character: true,
        }.as_packet_with_buf(&mut buf).unwrap();

        let _ = chunk.server.io_loop.send(Msg::Write(self.token, buf));
    }

    pub fn handle_identification(&mut self, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        use std::io::Read;
        use shared::io::ReadExt;

        if self.account.is_some() {
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

        let io_loop = chunk.server.io_loop.clone();
        let handler = chunk.server.handler.clone();
        let token = self.token;

        self.queue_size = QUEUE_SIZE.fetch_add(1, Ordering::Relaxed) + 1;
        self.queue_counter = QUEUE_COUNTER.load(Ordering::Relaxed);

        database::execute(&chunk.server.db, move |conn| {
            match Session::authenticate(conn, username, password) {
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

                        session.queue_size = -1;
                        session.queue_counter = -1;
                        Session::identification_success(session, chunk, data, already)
                    });
                }
            }

            let _ = QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed);
            let _ = QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
        });

        Ok(())
    }

    pub fn handle_server_selection(&mut self, chunk: &Chunk, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.account.is_none() {
            return Ok(());
        }

        let msg = try!(ServerSelectionMessage::deserialize(&mut data));

        let status = chunk.game_status.get(&(msg.server_id.0 as i16));
        if status.is_none() {
            return Ok(());
        }
        let ref status = status.unwrap();

        if status.0 != server_status::ONLINE && (status.0 != server_status::FULL ||
            !self.account.as_ref().unwrap().is_subscriber()) {

            return Ok(());
        }

        /*if server.min_level > self.account.as_ref().unwrap().level {
            return ();
        }*/

        let ticket: String = rand::thread_rng().gen_ascii_chars().take(10).collect();
        let mut output = [0; 16];
        //let mut result = Vec::new();

        let mut cbc = aes::cbc_encryptor(aes::KeySize::KeySize256, &self.aes_key[0..],
            &self.aes_key[0..16], PkcsPadding);

        let mut read_buffer = RefReadBuffer::new(&ticket.as_bytes());
        let mut write_buffer = RefWriteBuffer::new(&mut output);

        /*loop {
            let res = match cbc.encrypt(&mut read_buffer, &mut write_buffer, true) {
                Ok(res) => res,
                Err(e) => {

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
        }*/

        /*let buf = SelectedServerDataMessage {
            server_id: msg.server_id,
            address: "127.0.0.1".to_string(),
            port: 5555,
            can_create_new_character: true,
            ticket: VarIntVec(result),
        }.as_packet().unwrap();
        let _ = chunk.server.io_loop.send(Msg::WriteAndClose(self.token, buf));*/

        Ok(())
    }
}
