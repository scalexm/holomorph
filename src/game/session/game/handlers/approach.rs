use session::game::{Session, AccountData, GameState};
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
use std::collections::{HashSet, HashMap};
use character::CharacterMinimal;

pub static QUEUE_SIZE: AtomicIsize = ATOMIC_ISIZE_INIT;
pub static QUEUE_COUNTER: AtomicIsize = ATOMIC_ISIZE_INIT;

enum Error {
    Sql(postgres::error::Error),
    Other,
}

impl From<postgres::error::Error> for Error {
    fn from(err: postgres::error::Error) -> Error {
        Error::Sql(err)
    }
}

fn authenticate(conn: &mut Connection, ticket: String, server_id: i16, addr: String)
    -> Result<AccountData, Error> {

    let trans = try!(conn.transaction());

    let stmt = try!(trans.prepare_cached("SELECT id, nickname, secret_answer, level,
        subscription_end FROM accounts WHERE ticket = $1"));
    let rows = try!(stmt.query(&[&ticket]));

    if rows.len() == 0 {
        return Err(Error::Other);
    }

    let row = rows.get(0);
    let id: i32 = try!(row.get_opt("id"));

    let stmt = try!(trans.prepare_cached("UPDATE accounts SET logged = $1,
        last_server = $1 WHERE id = $2"));
    try!(stmt.execute(&[&server_id, &id]));

    let stmt = try!(trans.prepare_cached("SELECT date, ip FROM connections_history
        WHERE id = $1 ORDER BY date DESC LIMIT 1"));
    let rows = try!(stmt.query(&[&id]));

    let (last_connection, last_ip) = if rows.len() == 0 {
        (0, String::new())
    } else {
        let row = rows.get(0);
        (try!(row.get_opt("date")), try!(row.get_opt("ip")))
    };

    let stmt = try!(trans.prepare_cached("INSERT INTO connections_history(id, date, ip)
        VALUES($1, $2, $3)"));
     try!(stmt.execute(&[&id, &time::get_time().sec, &addr]));

    /*let stmt = try!(trans.prepare_cached("SELECT friends, warn_on_connection,
        warn_on_level_gain FROM friends WHERE "));*/

    try!(trans.commit());

    let level: i16 = try!(row.get_opt("level"));
    Ok(AccountData {
        id: id,
        nickname: try!(row.get_opt("nickname")),
        secret_answer: try!(row.get_opt("secret_answer")),
        level: level as i8,
        subscription_end: try!(row.get_opt("subscription_end")),
        last_connection: last_connection,
        last_ip: last_ip,
        friends: HashSet::new(),
        ignored: HashSet::new(),
    })
}

impl Session {
    fn identification_success(&mut self, data: AccountData,
        characters: HashMap<i32, CharacterMinimal>) {

        log_info!(self, "game connection: ip = {}", self.base.address);

        let mut buf = QueueStatusMessage {
            position: 0,
            total: 0,
        }.as_packet().unwrap();

        AuthenticationTicketAcceptedMessage.as_packet_with_buf(&mut buf).unwrap();

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
            can_create_new_character: Flag(characters.len() < 5),
            account_id: data.id,
            breeds_visible: -1,
            breeds_available: -1,
            status: 0,
        }.as_packet_with_buf(&mut buf).unwrap();

        TrustStatusMessage { // AnkamaShield
            trusted: Flag(true),
            certified: Flag(true),
        }.as_packet_with_buf(&mut buf).unwrap();

        write!(SERVER, self.base.token, buf);

        self.account = Some(data);
        self.state = GameState::CharacterSelection(characters);
    }

    pub fn handle_authentication_ticket<'a>(&mut self, _: Ref<'a>, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if !self.state.is_none() {
            return Ok(());
        }

        let msg = try!(AuthenticationTicketMessage::deserialize(&mut data));

        let ticket = msg.ticket;
        let (server_id, io_loop, server) = SERVER.with(|s| {
            (s.cnf.server_id, s.io_loop.clone(), s.server.clone())
        });
        let token = self.base.token;
        let addr = self.base.address.clone();

        self.state = GameState::TicketQueue(QUEUE_SIZE.fetch_add(1, Ordering::Relaxed)
            + 1, QUEUE_COUNTER.load(Ordering::Relaxed));

        SERVER.with(|s| database::execute(&s.auth_db, move |conn| {
            match authenticate(conn, ticket, server_id, addr) {
                Err(err) => {
                    if let Error::Sql(err) = err {
                        error!("authenticate sql error: {}", err);
                    }

                    let buf = AuthenticationTicketRefusedMessage.as_packet().unwrap();
                    let _ = io_loop.send(Msg::WriteAndClose(token, buf));
                }

                Ok(data) => {
                    let id = data.id;
                    server::identification_success(&server, token, id,
                        move |session, characters| {
                            session.identification_success(data, characters)
                    });
                }
            }

            let _ = QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed);
            let _ = QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
        }));

        Ok(())
    }
}
