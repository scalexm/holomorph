use session::game::{Session, QueueState};
use session::game::chunk::Ref;
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::messages::game::character::choice::*;
use shared::protocol::messages::game::inventory::items::*;
use shared::protocol::messages::game::character::stats::*;
use shared::protocol::messages::game::context::notification::*;
use shared::net::Msg;
use postgres::{self, Connection};
use character::{CharacterMinimal, Character};
use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};
use shared::database;
use server::{self, SERVER};

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

fn load_character(conn: &mut Connection, base: CharacterMinimal)
    -> Result<Character, Error> {

    let stmt = try!(conn.prepare_cached("SELECT * FROM characters WHERE id = $1"));
    let rows = try!(stmt.query(&[&base.id()]));

    if rows.len() == 0 {
        return Err(Error::Other);
    }

    Ok(try!(Character::from_sql(base, rows.get(0))))
}

impl Session {
    pub fn handle_characters_list_request<'a>(&mut self, _: Ref<'a>, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.account.is_none() || !self.queue_state.is_none() {
            return Ok(());
        }

        let buf = CharactersListMessage {
            base: BasicCharactersListMessage {
                characters: self.characters
                    .iter()
                    .map(|ch| ch.1.as_character_base().into())
                    .collect(),
            },
            has_startup_actions: false,
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);
        Ok(())
    }

    fn character_selection_success(&mut self, ch: Character) {
        self.current_character = Some(ch);
        self.queue_state = QueueState::None;
        let ch = self.current_character.as_ref().unwrap();

        let mut buf = CharacterSelectedSuccessMessage {
            infos: ch.minimal().as_character_base(),
            is_collecting_stats: false,
        }.as_packet().unwrap();

        InventoryContentMessage {
            objects: Vec::new(),
            kamas: VarInt(ch.kamas()),
        }.as_packet_with_buf(&mut buf).unwrap();

        InventoryWeightMessage {
            weight: VarInt(0),
            weight_max: VarInt(0),
        }.as_packet_with_buf(&mut buf).unwrap();

        NotificationListMessage {
            flags: Vec::new(),
        }.as_packet_with_buf(&mut buf).unwrap();

        CharacterStatsListMessage {
            stats: ch.get_character_characteristics(),
        }.as_packet_with_buf(&mut buf).unwrap();

        write!(SERVER, self.base.token, buf);
    }

    pub fn handle_character_selection<'a>(&mut self, _: Ref<'a>, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        if self.account.is_none() || !self.queue_state.is_none() {
            return Ok(());
        }

        let msg = try!(CharacterSelectionMessage::deserialize(&mut data));

        let ch = match self.characters.remove(&msg.id) {
            Some(ch) => ch,
            None => {
                let buf = CharacterSelectedErrorMessage.as_packet().unwrap();
                write!(SERVER, self.base.token, buf);
                return Ok(());
            }
        };

        let token = self.base.token;
        let server = SERVER.with(|s| s.server.clone());
        let io_loop = SERVER.with(|s| s.io_loop.clone());

        self.queue_state = QueueState::SomeGame(QUEUE_SIZE.fetch_add(1, Ordering::Relaxed)
            + 1, QUEUE_COUNTER.load(Ordering::Relaxed));

        SERVER.with(|s| database::execute(&s.db, move |conn| {
            match load_character(conn, ch) {
                Err(err) => {
                    if let Error::SqlError(err) = err {
                        error!("load_character sql error: {}", err);
                    }

                    let buf = CharacterSelectedErrorMessage.as_packet().unwrap();
                    let _ = io_loop.send(Msg::Write(token, buf));
                }

                Ok(ch) => {
                    let ch_id = ch.minimal().id();
                    server::character_selection_success(&server, token, ch_id,
                        move |session| session.character_selection_success(ch));
                }
            }

            let _ = QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed);
            let _ = QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
        }));

        Ok(())
    }
}
