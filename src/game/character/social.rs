use super::CharacterMinimal;
use session::game::SocialInformations;
use protocol::*;
use protocol::types::game::friend::*;
use protocol::types::game::social::*;
use protocol::types::game::context::roleplay::BasicGuildInformations;
use protocol::variants::{FriendInformationsVariant, IgnoredInformationsVariant};
use protocol::enums::player_status;

impl CharacterMinimal {
    pub fn as_ignored_infos(&self, social: Option<&SocialInformations>)
                            -> IgnoredInformationsVariant {
        let infos = IgnoredInformations {
            base: AbstractContactInformations {
                account_id: self.account_id,
                account_name: self.account_nickname.clone(),
            }
        };

        if social.is_none() {
            IgnoredInformationsVariant::IgnoredInformations(infos)
        } else {
            IgnoredInformationsVariant::IgnoredOnlineInformations(IgnoredOnlineInformations {
                base: infos,
                player_id: VarInt(self.id),
                player_name: self.name.clone(),
                breed: self.breed,
                sex: self.sex,
            })
        }
    }

    pub fn as_friend_infos(&self, account_id: i32, social: Option<&SocialInformations>)
                           -> FriendInformationsVariant {
        let is_friend_with = match social {
            None => false,
            Some(social) => social.is_friend_with(account_id),
        };

        let infos = FriendInformations {
            base: AbstractContactInformations {
                account_id: self.account_id,
                account_name: self.account_nickname.clone(),
            },
            player_state: match social {
                None => player_status::OFFLINE,
                Some(social) => social.status.status_id(),
            },
            last_connection: VarShort(0),
            achievement_points: if is_friend_with { 0 } else { -1 },
        };

        match social {
            None => FriendInformationsVariant::FriendInformations(infos),
            Some(social) => {
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
                    status: social.status.clone(),
                })
            },
        }
    }
}
