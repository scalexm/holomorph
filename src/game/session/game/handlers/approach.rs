use session::game::{Session, AccountData, GameState, SocialInformations};
use session::game::handlers::{UpdateSqlAccount, SqlRelations};
use session::game::handlers::error::Error;
use session::game::chunk::Ref;
use protocol::{Protocol, Flag};
use protocol::messages::game::approach::*;
use protocol::messages::queues::*;
use protocol::messages::game::basic::*;
use protocol::messages::secure::*;
use protocol::types::game::character::status::*;
use protocol::variants::PlayerStatusVariant;
use protocol::enums::player_status;
use std::io;
use shared::net::Msg;
use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};
use diesel::*;
use shared::database;
use server::{self, SERVER};
use time;
use std::collections::{HashMap};
use character::CharacterMinimal;
use shared::database::schema::connections_history;

pub static QUEUE_SIZE: AtomicIsize = ATOMIC_ISIZE_INIT;
pub static QUEUE_COUNTER: AtomicIsize = ATOMIC_ISIZE_INIT;

#[derive(Queriable)]
struct SqlAccount {
    id: i32,
    nickname: String,
    secret_answer: String,
    level: i16,
    subscription_end: i64,
}

#[derive(Queriable)]
#[insertable_into(connections_history)]
struct HistoryEntry {
    date: i64,
    ip: String,
    account_id: i32,
}

fn authenticate(conn: &Connection, ticket: String, server_id: i16, addr: String)
                -> Result<AccountData, Error> {
    use shared::database::schema::{accounts, social_relations};
    use diesel::query_builder::{insert, update};

    let account: Option<SqlAccount> = try!(
        accounts::table.filter(accounts::ticket.eq(&ticket))
                       .select((
                           accounts::id,
                           accounts::nickname,
                           accounts::secret_answer,
                           accounts::level,
                           accounts::subscription_end
                       )).first(conn).optional()
    );

    let account = match account {
        Some(account) => account,
        None => return Err(Error::Other),
    };

    let _ = try!(
        update(
            accounts::table.filter(accounts::id.eq(&account.id))
        ).set(&UpdateSqlAccount {
            already_logged: Some(server_id),
            last_server: Some(server_id),
        }).execute(conn)
    );

    let last_conn = try!(
        connections_history::table.filter(connections_history::account_id.eq(&account.id))
                                  .order(connections_history::date.desc())
                                  .select((
                                      connections_history::date,
                                      connections_history::ip,
                                      connections_history::account_id,
                                  )).first(conn).optional()
    ).unwrap_or(HistoryEntry { date: 0, ip: String::new(), account_id: 0 });

    let _ = try!(insert(&HistoryEntry {
        date: time::get_time().sec,
        ip: addr,
        account_id: account.id,
    }).into(connections_history::table).execute(conn));

    let social: Option<SqlRelations> = try!(
        social_relations::table.filter(social_relations::id.eq(&account.id))
                               .select((
                                   social_relations::friends,
                                   social_relations::ignored,
                                   social_relations::warn_on_connection,
                                   social_relations::warn_on_level_gain
                               )).first(conn).optional()
    );

    let social = match social {
        Some(social) => social,
        None => {
            error!("account id {} has no social relations", account.id);
            return Err(Error::Other);
        }
    };

    Ok(AccountData {
        id: account.id,
        nickname: account.nickname,
        secret_answer: account.secret_answer,
        level: account.level as i8,
        subscription_end: account.subscription_end,
        last_connection: last_conn.date,
        last_ip: last_conn.ip,
        social: SocialInformations {
            friends: social.friends.into_iter().collect(),
            ignored: social.ignored.into_iter().collect(),
            warn_on_connection: social.warn_on_connection,
            warn_on_level_gain: social.warn_on_level_gain,
            status: PlayerStatusVariant::PlayerStatus(PlayerStatus {
                status_id: player_status::AVAILABLE,
            }),
        },
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
            status: player_status::IDLE,
        }.as_packet_with_buf(&mut buf).unwrap();

        TrustStatusMessage { // AnkamaShield
            trusted: Flag(true),
            certified: Flag(true),
        }.as_packet_with_buf(&mut buf).unwrap();

        write!(SERVER, self.base.token, buf);

        self.account = Some(data);
        self.state = GameState::CharacterSelection(characters);
    }
}

#[register_handlers]
impl Session {
    pub fn handle_authentication_ticket<'a>(&mut self, _: Ref<'a>,
                                            msg: AuthenticationTicketMessage) -> io::Result<()> {
        if !self.state.is_none() {
            return Ok(());
        }

        let ticket = msg.ticket;
        let (server_id, io_loop, server) = SERVER.with(|s| {
            (s.cnf.server_id, s.io_loop.clone(), s.server.clone())
        });
        let token = self.base.token;
        let addr = self.base.address.clone();

        self.state = GameState::TicketQueue(QUEUE_SIZE.fetch_add(1, Ordering::Relaxed)
            + 1, QUEUE_COUNTER.load(Ordering::Relaxed));

        SERVER.with(|s| database::execute(&s.auth_db, move |conn| {
            let res = conn.transaction(|| {
                authenticate(conn, ticket, server_id, addr)
            }).map_err(From::from);

            match res {
                Err(err) => {
                    if let Error::Sql(err) = err {
                        error!("authenticate sql error: {}", err);
                    }

                    let buf = AuthenticationTicketRefusedMessage.as_packet().unwrap();
                    let _ = io_loop.send(Msg::WriteAndClose(token, buf));
                }

                Ok(data) => {
                    let id = data.id;
                    server::identification_success(
                        &server,
                        token,
                        id,
                        move |session, characters| {
                            session.identification_success(data, characters)
                        }
                    );
                }
            }

            let _ = QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed);
            let _ = QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
        }));

        Ok(())
    }
}
