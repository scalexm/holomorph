use session::game::{Session, GameState};
use session::game::chunk::{Ref, ChunkImpl};
use std::io;
use protocol::{Protocol, VarInt, VarShort};
use protocol::messages::game::character::choice::*;
use protocol::messages::game::character::creation::*;
use protocol::messages::game::inventory::items::*;
use protocol::messages::game::character::stats::*;
use protocol::messages::game::context::notification::*;
use protocol::messages::game::chat::channel::EnabledChannelsMessage;
use protocol::types::game::look::*;
use protocol::enums::{character_creation_result, chat_channels_multi};
use session::game::handlers::error::Error;
use protocol::variants::{FriendInformationsVariant, IgnoredInformationsVariant};
use shared::net::{Token, Msg};
use diesel::*;
use character::{CharacterMinimal, Character, SqlCharacter};
use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};
use shared::database;
use shared::database::schema::character_counts;
use server::{self, SERVER};
use std::collections::HashMap;
use protocol::messages::queues::*;
use std::mem;

pub static QUEUE_SIZE: AtomicIsize = ATOMIC_ISIZE_INIT;
pub static QUEUE_COUNTER: AtomicIsize = ATOMIC_ISIZE_INIT;

fn validate_name(name: &str) -> bool {
    if name.len() < 4 {
        return false;
    }
    let mut dash = false;
    let len = name.len();
    let name: Vec<_> = name.chars().collect();
    for i in 0..len {
        if name[i] == '-' {
            if dash || i == 0 || i == len - 1 {
                return false;
            }
            dash = true;
        } else if name[i] < 'A' || (name[i] > 'Z' && name[i] < 'a') || name[i] > 'z' {
            return false;
        }

        if name[i] >= 'A' && name[i] <= 'Z' && i != 0 && name[i - 1] != '-' {
            return false;
        }
    }
    true
}

#[insertable_into(character_counts)]
struct CharacterCount(
    #[column_name="server_id"]
    i16,
    #[column_name="account_id"]
    i32,
);

fn create_character(conn: &Connection, tok: Token, name: String, breed: i16, look: EntityLook,
                    sex: bool, spawn_map: i32, account_id: i32, nickname: String,
                    auth_uri: String, server_id: i16)
                    -> Result<CharacterMinimal, Error> {
    use shared::database::schema::{lower, characters, character_minimals};

    let lower_name = name.to_lowercase();
    let name_exists: Option<i32> = try!(
        character_minimals::table.filter(lower(character_minimals::name).eq(&lower_name))
                                 .select_sql::<types::Integer>("1")
                                 .first(conn)
                                 .optional()
    );

    if name_exists.is_some() {
        return Err(Error::Other);
    }

    let auth_conn = try!(Connection::establish(&auth_uri));
    auth_conn.transaction(|| {
        try!(
            insert(&CharacterCount(server_id, account_id)).into(character_counts::table)
                                                          .execute(&auth_conn)
        );

        let new_char = SqlCharacter::default(spawn_map);

        let res: SqlCharacter = try!(
            insert(&new_char).into(characters::table)
                             .get_result(conn)
        );
        let id = res.id();

        let base = CharacterMinimal::new(id, account_id, nickname, 1, name, breed, sex, look);
        try!(
            insert(&base).into(character_minimals::table)
                         .execute(conn)
        );

        Ok(base)
    }).map_err(From::from)
}

fn load_character(conn: &Connection, tok: Token, base: CharacterMinimal)
                  -> Result<(Character, i32), Error> {
    use shared::database::schema::characters;

    let ch_id = base.id();
    let character: Option<SqlCharacter> = try!(
        characters::table.filter(characters::id.eq(&ch_id))
                         .first(conn)
                         .optional()
    );

    match character {
        Some(character) => {
            let map_id = character.map_id;
            match Character::new(tok, base, character) {
                Some(character) => Ok((character, map_id)),
                None => {
                    error!("invalid cell for character {}", ch_id);
                    return Err(Error::Other);
                }
            }
        },
        None => return Err(Error::Other),
    }
}

enum SelectionType {
    Creation(String, i16, EntityLook, bool, i32, i32, String, String, i16),
    Default(CharacterMinimal),
}

struct ScopeExit;

impl Drop for ScopeExit {
    fn drop(&mut self) {
        let _ = QUEUE_SIZE.fetch_sub(1, Ordering::Relaxed);
        let _ = QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
}

impl Session {
    fn character_selection_error(&mut self, chars: HashMap<i64, CharacterMinimal>) {
        self.state = GameState::CharacterSelection(chars);

        let buf = QueueStatusMessage {
            position: 0,
            total: 0,
        }.unwrap();
        write!(SERVER, self.base.token, buf);
    }

    fn character_selection_success(&mut self, _: &mut ChunkImpl, mut ch: Character, map_id: i32,
                                   friends: HashMap<i32, FriendInformationsVariant>,
                                   ignored: HashMap<i32, IgnoredInformationsVariant>) {
        log_info!(self, "selected character id = {}", ch.minimal().id());

        let mut buf = CharacterSelectedSuccessMessage {
            infos: ch.minimal().as_character_base(),
            is_collecting_stats: false,
        }.unwrap();

        QueueStatusMessage {
            position: 0,
            total: 0,
        }.unwrap_with_buf(&mut buf);

        InventoryContentMessage {
            objects: Vec::new(),
            kamas: VarInt(ch.kamas()),
        }.unwrap_with_buf(&mut buf);

        InventoryWeightMessage {
            weight: VarInt(0),
            weight_max: VarInt(0),
        }.unwrap_with_buf(&mut buf);

        NotificationListMessage {
            flags: Vec::new(),
        }.unwrap_with_buf(&mut buf);

        CharacterStatsListMessage {
            stats: ch.get_character_characteristics(),
        }.unwrap_with_buf(&mut buf);

        let has_global_channel = {
            let account = self.account.as_ref().unwrap();
            EnabledChannelsMessage {
                channels: account.channels.iter().cloned().collect(),
                disallowed: Vec::new(),
            }.unwrap_with_buf(&mut buf);

            account.channels.contains(&(chat_channels_multi::GLOBAL as u8))
        };

        ch.set_has_global_channel(has_global_channel);

        write!(SERVER, self.base.token, buf);
        self.state = GameState::SwitchingContext(map_id, ch);
        self.friends_cache = friends;
        self.ignored_cache = ignored;
    }

    fn select_character(&mut self, ty: SelectionType) {
        let account = self.account.as_ref().unwrap();

        let token = self.base.token;
        let (server, io_loop) = SERVER.with(|s| {
            (s.server.clone(), s.io_loop.clone())
        });
        let account_id = account.id;
        let social = account.social.clone();

        let state = GameState::GameQueue(
            QUEUE_SIZE.fetch_add(1, Ordering::Relaxed) + 1,
            QUEUE_COUNTER.load(Ordering::Relaxed)
        );

        let state = mem::replace(&mut self.state, state);
        let characters_list = match state {
            GameState::CharacterSelection(characters) => characters,
            _ => unreachable!(),
        };

        SERVER.with(|s| database::execute(&s.db, move |conn| {
            let decrease_queue = ScopeExit;
            let base = match ty {
                SelectionType::Creation(
                    name, breed, look, sex, spawn_map,
                    account_id, account_nickname, auth_uri, server_id
                ) => {
                    let res = conn.transaction(|| {
                        create_character(
                            conn, token,
                            name, breed, look, sex, spawn_map,
                            account_id, account_nickname, auth_uri, server_id
                        )
                    }).map_err(From::from);

                    match res {
                        Err(err) => {
                            let result = if let Error::Sql(err) = err {
                                error!("create_character sql error: {}", err);
                                character_creation_result::ERR_NO_REASON
                            } else {
                                character_creation_result::ERR_NAME_ALREADY_EXISTS
                            };

                            let buf = CharacterCreationResultMessage {
                                result: result,
                            }.unwrap();
                            let _ = io_loop.send(Msg::Write(token, buf));
                            server::session_callback(&server, token, move |session, _| {
                                session.character_selection_error(characters_list)
                            });
                            return;
                        },

                        Ok(base) => {
                            let buf = CharacterCreationResultMessage {
                                result: character_creation_result::OK,
                            }.unwrap();
                            let _ = io_loop.send(Msg::Write(token, buf));
                            server::insert_character_minimal(&server, base.clone());
                            base
                        }
                    }
                },

                SelectionType::Default(base) => base,
            };

            let res = conn.transaction(|| {
                load_character(conn, token, base)
            }).map_err(From::from);

            match res {
                Err(err) => {
                    if let Error::Sql(err) = err {
                        error!("load_character sql error: {}", err);
                    }
                    let buf = CharacterSelectedErrorMessage.unwrap();
                    let _ = io_loop.send(Msg::Write(token, buf));
                    server::session_callback(&server, token, move |session, _| {
                        session.character_selection_error(characters_list)
                    });
                    return;
                }

                Ok((ch, map_id)) => {
                    let ch_id = ch.minimal().id();
                    server::character_selection_success(
                        &server,
                        token,
                        account_id,
                        ch_id,
                        social,
                        move |session, chunk, friends, ignored| {
                            session.character_selection_success(
                                chunk,
                                ch,
                                map_id,
                                friends,
                                ignored
                            )
                        }
                    );
                }
            }
        }));
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
        }.unwrap();

        write!(SERVER, self.base.token, buf);
        Ok(())
    }

    pub fn handle_character_creation_request<'a>(&mut self, chunk: Ref<'a>,
                                                 msg: CharacterCreationRequestMessage)
                                                 -> io::Result<()> {
        match self.state {
            GameState::CharacterSelection(_) => (),
            _ => return Ok(()),
        };

        let characters_count = self.account.as_ref().unwrap().characters_count;
        let max_characters_count = self.account.as_ref().unwrap().max_characters_count;
        if characters_count >= max_characters_count {
            let buf = CharacterCreationResultMessage {
                result: character_creation_result::ERR_TOO_MANY_CHARACTERS,
            }.unwrap();
            write!(SERVER, self.base.token, buf);
            return Ok(());
        }

        if !validate_name(&msg.name) {
            let buf = CharacterCreationResultMessage {
                result: character_creation_result::ERR_INVALID_NAME,
            }.unwrap();
            write!(SERVER, self.base.token, buf);
            return Ok(());
        }

        let look_and_map = SERVER.with(|s| {
            s.breeds.get(&(msg.breed as i16)).map(|b| {
                (
                    if msg.sex { b.female_look().clone() } else { b.male_look().clone() },
                    b.spawn_map()
                )
            })
        });

        let (mut look, map) = match look_and_map {
            Some((look, map)) => (look, map),
            None => {
                let buf = CharacterCreationResultMessage {
                    result: character_creation_result::ERR_NO_REASON,
                }.unwrap();
                write!(SERVER, self.base.token, buf);
                return Ok(());
            }
        };

        let head = SERVER.with(|s| {
            s.heads.get(&msg.cosmetic_id.0).map(|h| h.clone())
        });

        if head.is_none() || head.as_ref().unwrap().breed_id() != msg.breed as i16
            || head.as_ref().unwrap().gender() != msg.sex {

            let buf = CharacterCreationResultMessage {
                result: character_creation_result::ERR_NO_REASON,
            }.unwrap();
            write!(SERVER, self.base.token, buf);
            return Ok(());
        }

        let mut colors = HashMap::new();
        for &c in &look.indexed_colors {
            let _ = colors.insert((c >> 24) & 255, c & 16777215);
        }

        for i in 0 .. msg.colors.0.len() {
            let ind = (i + 1) as i32;
            if msg.colors.0[i] != -1 && colors.contains_key(&ind) {
                let _ = colors.insert(ind, msg.colors.0[i]);
            }
        }

        look.indexed_colors.clear();
        for p in &colors {
            look.indexed_colors.push((p.0 & 255) << 24 | p.1 & 16777215);
        }

        look.skins.push(VarShort(head.unwrap().skin()));

        let account_id = self.account.as_ref().unwrap().id;
        let account_nickname = self.account.as_ref().unwrap().nickname.clone();
        let (auth_uri, server_id) = SERVER.with(|s| {
            (s.cnf.auth_database_uri.clone(), s.cnf.server_id)
        });

        self.select_character(SelectionType::Creation(
            msg.name, msg.breed as i16, look, msg.sex, map,
            account_id, account_nickname, auth_uri, server_id
        ));

        Ok(())
    }

    pub fn handle_character_selection<'a>(&mut self, _: Ref<'a>, msg: CharacterSelectionMessage)
                                          -> io::Result<()> {
        let ch = {
            let characters = match self.state {
                GameState::CharacterSelection(ref characters) => characters,
                _ => return Ok(()),
            };

            match characters.get(&msg.id.0) {
                Some(ch) => ch.clone(),
                None => {
                    let buf = CharacterSelectedErrorMessage.unwrap();
                    write!(SERVER, self.base.token, buf);
                    return Ok(());
                }
            }
        };

        self.select_character(SelectionType::Default(ch));

        Ok(())
    }
}
