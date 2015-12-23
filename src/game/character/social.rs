use super::CharacterMinimal;
use session::game::{SocialState, SocialInformations};
use protocol::*;
use protocol::types::game::friend::*;
use protocol::types::game::social::*;
use protocol::types::game::context::roleplay::BasicGuildInformations;
use protocol::variants::{FriendInformationsVariant, IgnoredInformationsVariant};
use protocol::enums::player_status;

pub enum RelationInformations {
    Friend(FriendInformationsVariant),
    Ignored(IgnoredInformationsVariant),
}

impl RelationInformations {
    pub fn as_friend(self) -> FriendInformationsVariant {
        match self {
            RelationInformations::Friend(f) => f,
            _ => panic!("not a friend"),
        }
    }

    pub fn as_ignored(self) -> IgnoredInformationsVariant {
        match self {
            RelationInformations::Ignored(i) => i,
            _ => panic!("not ignored"),
        }
    }
}

impl CharacterMinimal {
    fn as_abstract_contact_informations(&self) -> AbstractContactInformations {
        AbstractContactInformations {
            account_id: self.account_id,
            account_name: self.account_nickname.clone(),
        }
    }

    fn as_ignored_infos(&self, social: Option<&SocialInformations>)
                        -> IgnoredInformationsVariant {
        let infos = IgnoredInformations {
            base: self.as_abstract_contact_informations(),
        };

        if social.is_none() {
            IgnoredInformationsVariant::IgnoredInformations(infos)
        } else {
            IgnoredInformationsVariant::IgnoredOnlineInformations(IgnoredOnlineInformations {
                base: infos,
                player_id: VarInt(self.id),
                player_name: self.name.clone(),
                breed: self.breed as i8,
                sex: self.sex,
            })
        }
    }

    fn as_friend_infos(&self, asker_id: i32, social: Option<&SocialInformations>)
                       -> FriendInformationsVariant {
        let is_friend_with = match social {
            None => false,
            Some(social) => social.has_relation_with(asker_id, SocialState::Friend),
        };

        let infos = FriendInformations {
            base: self.as_abstract_contact_informations(),
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
                    breed: self.breed as i8,
                    sex: self.sex,
                    guild_info: BasicGuildInformations {
                        base: AbstractSocialGroupInfos,
                        guild_id: VarInt(0),
                        guild_name: String::new(),
                    },
                    mood_smiley_id: VarShort(if is_friend_with { self.mood_smiley } else { 0 }),
                    status: social.status.clone(),
                })
            },
        }
    }

    pub fn as_relation_infos(&self, asker_id: i32, social: Option<&SocialInformations>,
                             st: SocialState) -> RelationInformations {
        match st {
            SocialState::Friend =>
                RelationInformations::Friend(
                    self.as_friend_infos(asker_id, social)
                ),
            SocialState::Ignored =>
                RelationInformations::Ignored(
                    self.as_ignored_infos(social)
                ),
        }
    }
}
