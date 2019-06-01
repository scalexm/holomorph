use crate::types::game::context::roleplay::GuildInformations;
use crate::types::game::look::EntityLook;
use crate::variants::PlayerStatusVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 92)]
pub struct FriendOnlineInformations<'a> {
    pub base: FriendInformations<'a>,
    #[protocol(flag)]
    pub sex: bool,
    #[protocol(flag)]
    pub haven_bag_shared: bool,
    #[protocol(var)]
    pub player_id: u64,
    pub player_name: &'a str,
    #[protocol(var)]
    pub level: u16,
    pub alignment_side: i8,
    pub breed: i8,
    pub guild_info: GuildInformations<'a>,
    #[protocol(var)]
    pub mood_smiley_id: u16,
    pub status: PlayerStatusVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 105)]
pub struct IgnoredOnlineInformations<'a> {
    pub base: IgnoredInformations<'a>,
    #[protocol(var)]
    pub player_id: u64,
    pub player_name: &'a str,
    pub breed: i8,
    pub sex: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 78)]
pub struct FriendInformations<'a> {
    pub base: AbstractContactInformations<'a>,
    pub player_state: u8,
    #[protocol(var)]
    pub last_connection: u16,
    pub achievement_points: i32,
    #[protocol(var)]
    pub league_id: i16,
    pub ladder_position: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 380)]
pub struct AbstractContactInformations<'a> {
    pub account_id: u32,
    pub account_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 77)]
pub struct FriendSpouseInformations<'a> {
    pub spouse_account_id: u32,
    #[protocol(var)]
    pub spouse_id: u64,
    pub spouse_name: &'a str,
    #[protocol(var)]
    pub spouse_level: u16,
    pub breed: i8,
    pub sex: i8,
    pub spouse_entity_look: EntityLook<'a>,
    pub guild_info: GuildInformations<'a>,
    pub alignment_side: i8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 561)]
pub struct AcquaintanceInformation<'a> {
    pub base: AbstractContactInformations<'a>,
    pub player_state: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 562)]
pub struct AcquaintanceOnlineInformation<'a> {
    pub base: AcquaintanceInformation<'a>,
    #[protocol(var)]
    pub player_id: u64,
    pub player_name: &'a str,
    #[protocol(var)]
    pub mood_smiley_id: u16,
    pub status: PlayerStatusVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 106)]
pub struct IgnoredInformations<'a> {
    pub base: AbstractContactInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 93)]
pub struct FriendSpouseOnlineInformations<'a> {
    pub base: FriendSpouseInformations<'a>,
    #[protocol(flag)]
    pub in_fight: bool,
    #[protocol(flag)]
    pub follow_spouse: bool,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
}
