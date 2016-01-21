use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
use types::game::context::roleplay::GuildInformations; 
impl_type!(HouseGuildNoneMessage, 5701, house_id| VarInt);
impl_type!(HouseGuildRightsMessage, 5703, house_id| VarInt, guild_info| GuildInformations, rights| VarInt);
impl_type!(HouseGuildRightsViewMessage, 5700);
impl_type!(HouseGuildShareRequestMessage, 5704, enable| bool, rights| VarInt);
