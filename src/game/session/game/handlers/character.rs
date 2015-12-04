use session::game::{Session, GameState};
use session::game::chunk::{Ref, ChunkImpl};
use std::io::{self, Cursor};
use protocol::{Protocol, VarInt};
use protocol::messages::game::character::choice::*;
use protocol::messages::game::inventory::items::*;
use protocol::messages::game::character::stats::*;
use protocol::messages::game::context::notification::*;
use protocol::variants::{FriendInformationsVariant, IgnoredInformationsVariant};
use shared::net::{Token, Msg};
use postgres::{self, Connection};
use character::{CharacterMinimal, Character};
use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};
use shared::database;
use server::{self, SERVER};
use std::collections::HashMap;
use protocol::messages::queues::*;

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

fn load_character(conn: &mut Connection, tok: Token, base: CharacterMinimal)
                  -> Result<(Character, i32), Error> {
    let stmt = try!(conn.prepare_cached("SELECT * FROM characters WHERE id = $1"));
    let rows = try!(stmt.query(&[&base.id()]));

    if rows.len() == 0 {
        return Err(Error::Other);
    }

    let row = rows.get(0);
    let map_id: i32 = try!(row.get_opt("map_id"));
    Ok((try!(Character::from_sql(tok, base, row)), map_id))
}

impl Session {
    fn character_selection_success(&mut self, _: &mut ChunkImpl, ch: Character, map_id: i32,
                                   friends: HashMap<i32, FriendInformationsVariant>,
                                   ignored: HashMap<i32, IgnoredInformationsVariant>) {
        log_info!(self, "selected character id = {}", ch.minimal().id());

        let mut buf = CharacterSelectedSuccessMessage {
            infos: ch.minimal().as_character_base(),
            is_collecting_stats: false,
        }.as_packet().unwrap();

        QueueStatusMessage {
            position: 0,
            total: 0,
        }.as_packet_with_buf(&mut buf).unwrap();

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
        self.state = GameState::SwitchingContext(map_id, ch);
        self.friends = friends;
        self.ignored = ignored;
    }
}

#[register_handlers]
impl Session {
    pub fn handle_characters_list_request<'a>(&mut self, _: Ref<'a>,
                                              _: CharactersListRequestMessage) -> io::Result<()> {
        let characters = match self.state {
            GameState::CharacterSelection(ref characters) => characters,
            _ => return Ok(()),
        };

        let buf = CharactersListMessage {
            base: BasicCharactersListMessage {
                characters: characters.iter()
                                      .map(|ch| ch.1.as_character_base().into())
                                      .collect(),
            },
            has_startup_actions: false,
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);
        Ok(())
    }

    pub fn handle_character_selection<'a>(&mut self, _: Ref<'a>, msg: CharacterSelectionMessage)
                                          -> io::Result<()> {
        let ch = {
            let characters = match self.state {
                GameState::CharacterSelection(ref mut characters) => characters,
                _ => return Ok(()),
            };

            match characters.remove(&msg.id) {
                Some(ch) => ch,
                None => {
                    let buf = CharacterSelectedErrorMessage.as_packet().unwrap();
                    write!(SERVER, self.base.token, buf);
                    return Ok(());
                }
            }
        };

        let account = self.account.as_ref().unwrap();

        let token = self.base.token;
        let (server, io_loop) = SERVER.with(|s| {
            (s.server.clone(), s.io_loop.clone())
        });
        let account_id = account.id;
        let social = account.social.clone();

        self.state = GameState::GameQueue(QUEUE_SIZE.fetch_add(1, Ordering::Relaxed) + 1,
                                          QUEUE_COUNTER.load(Ordering::Relaxed));

        SERVER.with(|s| database::execute(&s.db, move |conn| {
            match load_character(conn, token, ch) {
                Err(err) => {
                    if let Error::Sql(err) = err {
                        error!("load_character sql error: {}", err);
                    }
                    let _ = io_loop.send(Msg::Close(token));
                }

                Ok((ch, map_id)) => {
                    let ch_id = ch.minimal().id();
                    server::character_selection_success(&server,
                                                        token,
                                                        account_id,
                                                        ch_id,
                                                        social,
                        move |session, chunk, friends, ignored| {
                            session.character_selection_success(chunk, ch, map_id,
                                                                friends, ignored)
                        });
                }
            }

            let _ = QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed);
            let _ = QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
        }));

        Ok(())
    }
}
