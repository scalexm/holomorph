use session::game::{Session, AccountData, GameState, SocialInformations};
use session::game::handlers::{UpdateSqlAccount, SqlRelations};
use session::game::handlers::error::Error;
use session::game::chunk::Ref;
use protocol::{Protocol, Flag, VarInt};
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
use shared::database::schema::{social_relations, connections_history};

pub static QUEUE_SIZE: AtomicIsize = ATOMIC_ISIZE_INIT;
pub static QUEUE_COUNTER: AtomicIsize = ATOMIC_ISIZE_INIT;

#[derive(Queryable)]
struct SqlAccount {
    id: i32,
    nickname: String,
    secret_answer: String,
    level: i16,
    subscription_end: i64,
    channels: Vec<i16>,
    max_characters_count: i16,
}

#[derive(Queryable)]
#[insertable_into(connections_history)]
struct HistoryEntry {
    date: i64,
    ip: String,
    account_id: i32,
}

#[insertable_into(social_relations)]
struct InsertSocial(
    #[column_name="id"]
    i32,
);

fn authenticate(conn: &Connection, ticket: String, server_id: i16, addr: String)
                -> Result<AccountData, Error> {
    use shared::database::schema::{accounts, character_counts};

    let account: Option<SqlAccount> = try!(
        accounts::table.filter(accounts::ticket.eq(&ticket))
                       .select((
                           accounts::id,
                           accounts::nickname,
                           accounts::secret_answer,
                           accounts::level,
                           accounts::subscription_end,
                           accounts::channels,
                           accounts::max_characters_count,
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
            channels: None,
        }).execute(conn)
    );

    let characters_count = try!(
        character_counts::table.filter(character_counts::account_id.eq(&account.id))
                               .select_sql::<types::BigInt>("COUNT(*)")
                               .first::<i64>(conn)
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
            try!(insert(&InsertSocial(account.id)).into(social_relations::table).execute(conn));
            SqlRelations {
                friends: Vec::new(),
                ignored: Vec::new(),
                warn_on_connection: false,
                warn_on_level_gain: false,
            }
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
        channels: account.channels.into_iter().map(|c| c as u8).collect(),
        characters_count: characters_count as i8,
        max_characters_count: account.max_characters_count as i8,
    })
}

impl Session {
    fn identification_success(&mut self, data: AccountData,
                             characters: HashMap<i64, CharacterMinimal>) {
        log_info!(self, "game connection: ip = {}", self.base.address);

        let mut buf = QueueStatusMessage {
            position: 0,
            total: 0,
        }.unwrap();

        AuthenticationTicketAcceptedMessage.unwrap_with_buf(&mut buf);

        BasicTimeMessage {
            timestamp: (time::get_time().sec * 1000) as f64,
            timezone_offset: (time::now().tm_utcoff / 60) as i16,
        }.unwrap_with_buf(&mut buf);

        ServerSettingsMessage {
            lang: "fr".to_string(),
            community: 0,
            game_type: 0,
        }.unwrap_with_buf(&mut buf);

        ServerOptionalFeaturesMessage {
            features: Vec::new(),
        }.unwrap_with_buf(&mut buf);

        ServerSessionConstantsMessage {
            variables: Vec::new(),
        }.unwrap_with_buf(&mut buf);

        AccountCapabilitiesMessage {
            tutorial_available: Flag(false),
            can_create_new_character: Flag(data.characters_count < data.max_characters_count),
            account_id: data.id,
            breeds_visible: VarInt(131071),
            breeds_available: VarInt(131071),
            status: player_status::IDLE,
        }.unwrap_with_buf(&mut buf);

        TrustStatusMessage { // AnkamaShield
            trusted: Flag(true),
            certified: Flag(true),
        }.unwrap_with_buf(&mut buf);

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

                    let buf = AuthenticationTicketRefusedMessage.unwrap();
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
