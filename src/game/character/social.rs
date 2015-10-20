use super::CharacterMinimal;
use session::game::SocialInformations;
use protocol::*;
use protocol::types::game::friend::*;
use protocol::types::game::social::*;
use protocol::types::game::context::roleplay::BasicGuildInformations;
use protocol::variants::{FriendInformationsVariant, IgnoredInformationsVariant};
use protocol::enums::player_status;
use session::game::chunk::SocialState;

impl CharacterMinimal {
    pub fn set_social(&mut self, infos: SocialInformations) {
        self.social = Some(infos);
    }

    pub fn is_friend_with(&self, account_id: i32) -> bool {
        self.social.as_ref().unwrap().friends.contains(&account_id)
    }

    pub fn ignores(&self, account_id: i32) -> bool {
        self.social.as_ref().unwrap().ignored.contains(&account_id)
    }

    pub fn as_ignored_infos(&self, state: SocialState) -> IgnoredInformationsVariant {
        let infos = IgnoredInformations {
            base: AbstractContactInformations {
                account_id: self.account_id,
                account_name: self.account_nickname.clone(),
            }
        };

        match state {
            SocialState::Offline => IgnoredInformationsVariant::IgnoredInformations(infos),
            _ => {
                IgnoredInformationsVariant::IgnoredOnlineInformations(IgnoredOnlineInformations {
                    base: infos,
                    player_id: VarInt(self.id),
                    player_name: self.name.clone(),
                    breed: self.breed,
                    sex: self.sex,
                })
            }
        }
    }

    pub fn as_friend_infos(&self, account_id: i32, state: SocialState)
                           -> FriendInformationsVariant {
        let is_friend_with = match state {
            SocialState::Offline => false,
            _ => self.is_friend_with(account_id),
        };

        let infos = FriendInformations {
            base: AbstractContactInformations {
                account_id: self.account_id,
                account_name: self.account_nickname.clone(),
            },
            player_state: match state {
                SocialState::Offline => player_status::OFFLINE,
                _ => self.social.as_ref().unwrap().status.status_id(),
            },
            last_connection: VarShort(0),
            achievement_points: if is_friend_with { 0 } else { -1 },
        };

        match state {
            SocialState::Offline => FriendInformationsVariant::FriendInformations(infos),
            _ => {
                FriendInformationsVariant::FriendOnlineInformations(FriendOnlineInformations {
                    base: infos,
                    player_id: VarInt(self.id),
                    player_name: self.name.clone(),
                    level: if is_friend_with { self.level as i8 } else { 0 },
                    alignment_side: 0,
                    breed: self.breed,
                    sex: self.sex,
                    guild_info: BasicGuildInformations {
                        base: AbstractSocialGroupInfos,
                        guild_id: VarInt(0),
                        guild_name: String::new(),
                    },
                    mood_smiley_id: VarShort(self.mood_smiley),
                    status: self.social.as_ref().unwrap().status.clone(),
                })
            },
        }
    }
}
