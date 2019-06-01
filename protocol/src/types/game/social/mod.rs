use crate::types::game::context::roleplay::AllianceInformations;
use crate::types::game::context::roleplay::BasicNamedAllianceInformations;
use crate::types::game::context::roleplay::GuildInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 416)]
pub struct AbstractSocialGroupInfos<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 421)]
pub struct AllianceFactSheetInformations<'a> {
    pub base: AllianceInformations<'a>,
    pub creation_date: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 422)]
pub struct AlliancedGuildFactSheetInformations<'a> {
    pub base: GuildInformations<'a>,
    pub alliance_infos: BasicNamedAllianceInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 437)]
pub struct GuildInAllianceVersatileInformations<'a> {
    pub base: GuildVersatileInformations<'a>,
    #[protocol(var)]
    pub alliance_id: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 423)]
pub struct GuildInsiderFactSheetInformations<'a> {
    pub base: GuildFactSheetInformations<'a>,
    pub leader_name: &'a str,
    #[protocol(var)]
    pub nb_connected_members: u16,
    pub nb_tax_collectors: u8,
    pub last_activity: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 424)]
pub struct GuildFactSheetInformations<'a> {
    pub base: GuildInformations<'a>,
    #[protocol(var)]
    pub leader_id: u64,
    #[protocol(var)]
    pub nb_members: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 435)]
pub struct GuildVersatileInformations<'a> {
    #[protocol(var)]
    pub guild_id: u32,
    #[protocol(var)]
    pub leader_id: u64,
    pub guild_level: u8,
    pub nb_members: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 432)]
pub struct AllianceVersatileInformations<'a> {
    #[protocol(var)]
    pub alliance_id: u32,
    #[protocol(var)]
    pub nb_guilds: u16,
    #[protocol(var)]
    pub nb_members: u16,
    #[protocol(var)]
    pub nb_subarea: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
