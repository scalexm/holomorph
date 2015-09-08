use session::game::{Session, AccountData, QueueState};
use session::game::chunk::Ref;
use shared::protocol::*;
use shared::protocol::messages::game::approach::*;
use shared::protocol::messages::queues::*;
use shared::protocol::messages::game::basic::*;
use shared::protocol::messages::secure::*;
use std::io::{self, Cursor};
use shared::net::Msg;
use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};
use postgres::{self, Connection};
use shared::database;
use server::{self, SERVER};
use time;
use std::collections::HashMap;
use character::CharacterMinimal;

pub static QUEUE_SIZE: AtomicIsize = ATOMIC_ISIZE_INIT;
pub static QUEUE_COUNTER: AtomicIsize = ATOMIC_ISIZE_INIT;

enum Error {
    SqlError(postgres::error::Error),
    Other,
}

impl From<postgres::error::Error> for Error {
    fn from(err: postgres::error::Error) -> Error {
        Error::SqlError(err)
    }
}

fn authenticate(conn: &mut Connection, ticket: String, server_id: i16, addr: String)
    -> Result<AccountData, Error> {

    let trans = try!(conn.transaction());

    let stmt = try!(trans.prepare_cached("SELECT id, nickname, secret_answer, level,
        subscription_end, last_connection_date, last_ip FROM accounts WHERE ticket = $1"));
    let rows = try!(stmt.query(&[&ticket]));

    if rows.len() == 0 {
        return Err(Error::Other);
    }

    let row = rows.get(0);

    let id: i32 = try!(row.get_opt("id"));

    let stmt = try!(trans.prepare_cached("UPDATE accounts SET logged = $1,
        last_server = $1, last_connection_date = $2, last_ip = $3 WHERE id = $4"));
    try!(stmt.execute(&[&server_id, &time::get_time().sec, &addr, &id]));

    try!(trans.commit());

    let level: i16 = try!(row.get_opt("level"));
    Ok(AccountData {
        id: id,
        nickname: try!(row.get_opt("nickname")),
        secret_answer: try!(row.get_opt("secret_answer")),
        level: level as i8,
        subscription_end: try!(row.get_opt("subscription_end")),
        last_connection: try!(row.get_opt("last_connection_date")),
        last_ip: try!(row.get_opt("last_ip")),
    })
}

impl Session {
    fn identification_success(&mut self, data: AccountData, already_logged: bool,
        characters: HashMap<i32, CharacterMinimal>) {

        if already_logged {
            let buf = AlreadyConnectedMessage.as_packet().unwrap();
            write!(SERVER, self.base.token, buf);
            return ();
        }

        self.account = Some(data);
        self.queue_state = QueueState::None;
        self.characters = characters;

        let mut buf = AuthenticationTicketAcceptedMessage.as_packet().unwrap();

        QueueStatusMessage {
            position: 0,
            total: 0,
        }.as_packet_with_buf(&mut buf).unwrap();

        BasicTimeMessage {
            timestamp: (time::get_time().sec * 1000) as f64,
            timezone_offset: (time::now().tm_utcoff / 60) as i16,
        }.as_packet_with_buf(&mut buf).unwrap();

        ServerSettingsMessage {
            lang: "fr".to_string(),
            community: 0,
            game_type: 0,
        }.as_packet_with_buf(&mut buf).unwrap();

        ServerOptionalFeaturesMessage {
            features: Vec::new(),
        }.as_packet_with_buf(&mut buf).unwrap();

        ServerSessionConstantsMessage {
            variables: Vec::new(),
        }.as_packet_with_buf(&mut buf).unwrap();

        AccountCapabilitiesMessage {
            tutorial_available: Flag(false),
            can_create_new_character: Flag(self.characters.len() < 5),
            account_id: self.account.as_ref().unwrap().id,
            breeds_visible: -1,
            breeds_available: -1,
            status: 0,
        }.as_packet_with_buf(&mut buf).unwrap();

        TrustStatusMessage { // AnkamaShield
            trusted: Flag(true),
            certified: Flag(true),
        }.as_packet_with_buf(&mut buf).unwrap();

        write!(SERVER, self.base.token, buf);
    }

    pub fn handle_authentication_ticket<'a>(&mut self, _: Ref<'a>,
        mut data: Cursor<Vec<u8>>) -> io::Result<()> {

        if self.account.is_some() || !self.queue_state.is_none() {
            return Ok(());
        }

        let msg = try!(AuthenticationTicketMessage::deserialize(&mut data));

        let ticket = msg.ticket;
        let server_id = SERVER.with(|s| s.cnf.server_id);
        let io_loop = SERVER.with(|s| s.io_loop.clone());
        let server = SERVER.with(|s| s.server.clone());
        let token = self.base.token;
        let addr = self.base.address.clone();

        self.queue_state = QueueState::SomeTicket(QUEUE_SIZE.fetch_add(1, Ordering::Relaxed)
            + 1, QUEUE_COUNTER.load(Ordering::Relaxed));

        SERVER.with(|s| database::execute(&s.auth_db, move |conn| {
            match authenticate(conn, ticket, server_id, addr) {
                Err(err) => {
                    if let Error::SqlError(err) = err {
                        error!("authenticate sql error: {}", err);
                    }

                    let buf = AuthenticationTicketRefusedMessage.as_packet().unwrap();
                    let _ = io_loop.send(Msg::WriteAndClose(token, buf));
                }

                Ok(data) => {
                    let id = data.id;
                    server::identification_success(&server, token, id,
                        move |session, already, characters| {
                            session.identification_success(data, already, characters)
                    });
                }
            }

            let _ = QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed);
            let _ = QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
        }));

        Ok(())
    }
}
