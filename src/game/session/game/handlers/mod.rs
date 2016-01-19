mod approach;
mod character;
mod friend;
mod context;
mod chat;
mod authorized;
mod player_status;
mod error;

use super::{Session, GameState, AccountData, SocialInformations, SocialState};
use super::chunk::{ChunkImpl, Ref, SocialUpdateType};
use character::CharacterMinimal;
use protocol::{Protocol, VarShort};
use protocol::messages::handshake::*;
use protocol::messages::game::approach::*;
use protocol::messages::queues::*;
use protocol::messages::game::basic::TextInformationMessage;
use protocol::enums::text_information_type;
use std::io::{self, Result};
use std::sync::atomic::Ordering;
use shared::{self, database};
use diesel::*;
use server::SERVER;
use character::Character;
use std::mem;
use std::collections::HashMap;
use shared::database::schema::{accounts, social_relations};

impl shared::session::Session<ChunkImpl> for Session {
    fn new(base: shared::session::SessionBase) -> Self {
        let mut buf = ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.as_packet().unwrap();

        HelloGameMessage.as_packet_with_buf(&mut buf).unwrap();

        write!(SERVER, base.token, buf);

        Session {
            base: base,
            account: None,
            state: GameState::None,

            last_sales_chat_request: 0,
            last_seek_chat_request: 0,

            friends_cache: HashMap::new(),
            ignored_cache: HashMap::new(),
        }
    }

    fn handle<'a>(&mut self, chunk: Ref<'a>, id: i16, mut data: io::Cursor<Vec<u8>>)
                  -> Result<()> {
        use protocol::messages::game::friend::{
            FriendsGetListMessage,
            FriendSetWarnOnConnectionMessage,
            FriendSetWarnOnLevelGainMessage,
            IgnoredGetListMessage,
            FriendAddRequestMessage,
            FriendDeleteRequestMessage,
            IgnoredAddRequestMessage,
            IgnoredDeleteRequestMessage,
        };
        use protocol::messages::game::character::status::PlayerStatusUpdateRequestMessage;
        use protocol::messages::game::chat::{
            ChatClientMultiMessage,
            ChatClientMultiWithObjectMessage,
            ChatClientPrivateMessage,
            ChatClientPrivateWithObjectMessage,
        };
        use protocol::messages::game::chat::channel::ChannelEnablingMessage;
        use protocol::messages::game::chat::smiley::{
            ChatSmileyRequestMessage,
            MoodSmileyRequestMessage,
        };
        use protocol::messages::game::character::choice::{
            CharactersListRequestMessage,
            CharacterSelectionMessage,
        };
        use protocol::messages::authorized::{
            AdminQuietCommandMessage,
        };
        use protocol::messages::game::context::{
            GameContextCreateRequestMessage,
            GameMapMovementRequestMessage,
            GameMapMovementCancelMessage,
            GameMapMovementConfirmMessage,
        };
        use protocol::messages::game::context::roleplay::{
            MapInformationsRequestMessage,
            ChangeMapMessage,
        };

        handle!(self, chunk, id, data)
    }

    fn close<'a>(mut self, mut chunk: Ref<'a>) {
        let account = mem::replace(&mut self.account, None);
        if let Some(account) = account {
            SERVER.with(|s| database::execute(&s.auth_db, move |conn| {
                if let Err(err) = Session::save_auth(conn, account) {
                    error!("error while saving session to auth db: {:?}", err);
                }
            }));
        }

        let state = mem::replace(&mut self.state, GameState::None);
        if let GameState::InContext(ch) = state {
            let map_id = ch.map_id;
            let ch = chunk.maps
                          .get_mut(&ch.map_id).unwrap()
                          .remove_actor(ch.id).unwrap()
                          .into_character();

            SERVER.with(|s| database::execute(&s.db, move |conn| {
                if let Err(err) = self.base.save_logs(conn, ch.minimal().account_id()) {
                    error!("error while saving logs: {:?}", err);
                }

                if let Err(err) = self.save_game(conn, ch, map_id) {
                    error!("error while saving session to game db: {:?}", err);
                }
            }));
        }
    }
}

impl Session {
    pub fn update_queue(&self) {
        let (global_queue_size, global_queue_counter) = match self.state {
            GameState::TicketQueue(..) => {
                use self::approach::{QUEUE_COUNTER, QUEUE_SIZE};
                (QUEUE_COUNTER.load(Ordering::Relaxed), QUEUE_SIZE.load(Ordering::Relaxed))
            }

            GameState::GameQueue(..) => {
                use self::character::{QUEUE_COUNTER, QUEUE_SIZE};
                (QUEUE_COUNTER.load(Ordering::Relaxed), QUEUE_SIZE.load(Ordering::Relaxed))
            }

            _ => return (),
        };

        let (former_queue_size, former_queue_counter) = match self.state {
            GameState::TicketQueue(qs, qc) | GameState::GameQueue(qs, qc) => (qs, qc),
            _ => unreachable!(),
        };

        let mut pos = former_queue_size - (global_queue_counter - former_queue_counter);

        if pos < 0 {
            pos = 0;
        }

        let buf = QueueStatusMessage {
            position: pos as i16,
            total: global_queue_size as i16,
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);
    }

    pub fn update_social(&mut self, ch: &CharacterMinimal, social: Option<&SocialInformations>,
                         ty: SocialUpdateType) {
        let account = match self.account.as_ref() {
            Some(account) => account,
            None => return,
        };

        let account_id = ch.account_id();

        if account.social.has_relation_with(account_id, SocialState::Friend) {
            let _ = self.friends_cache.insert(
                account_id,
                ch.as_relation_infos(account.id, social, SocialState::Friend).as_friend()
            );

            match ty {
                SocialUpdateType::Online if account.social.warn_on_connection => {
                    let buf = TextInformationMessage {
                        msg_type: text_information_type::MESSAGE,
                        msg_id: VarShort(143),
                        parameters: vec![ch.name().to_string(), ch.account_nickname().to_string(),
                                         account_id.to_string()],
                    }.as_packet().unwrap();
                    write!(SERVER, self.base.token, buf);
                },

                SocialUpdateType::WithLevel(_) if account.social.warn_on_level_gain => {
                    // TODO
                },

                _ => (),
            }
        }

        if account.social.has_relation_with(account_id, SocialState::Ignored) {
            let _ = self.ignored_cache.insert(
                account_id,
                ch.as_relation_infos(account.id, social, SocialState::Ignored).as_ignored()
            );
        }
    }
}

#[changeset_for(accounts)]
struct UpdateSqlAccount {
    already_logged: Option<i16>,
    last_server: Option<i16>,
    channels: Option<Vec<i16>>,
}

#[derive(Queryable)]
#[changeset_for(social_relations)]
struct SqlRelations {
    friends: Vec<i32>,
    ignored: Vec<i32>,
    warn_on_connection: bool,
    warn_on_level_gain: bool,
}

impl Session {
    fn save_auth(conn: &Connection, account: AccountData) -> QueryResult<()> {
        try!(conn.transaction(move || {
            let _ = try!(
                update(
                    accounts::table.filter(accounts::id.eq(&account.id))
                ).set(&UpdateSqlAccount {
                    already_logged: Some(0),
                    last_server: None,
                    channels: Some(account.channels.into_iter().map(|c| c as i16).collect()),
                }).execute(conn)
            );

            let _ = try!(
                update(
                    social_relations::table.filter(social_relations::id.eq(&account.id))
                ).set(&SqlRelations {
                    friends: account.social.friends.into_iter().collect(),
                    ignored: account.social.ignored.into_iter().collect(),
                    warn_on_connection: account.social.warn_on_connection,
                    warn_on_level_gain: account.social.warn_on_level_gain,
                }).execute(conn)
            );
            Ok(())
        }));
        Ok(())
    }

    fn save_game(&self, conn: &Connection, ch: Character, map: i32) -> QueryResult<()> {
        try!(conn.transaction(|| {
            ch.save(conn, map)
        }));
        Ok(())
    }
}
