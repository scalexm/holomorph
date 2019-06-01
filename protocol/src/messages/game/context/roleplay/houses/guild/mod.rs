use crate::types::game::context::roleplay::GuildInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5703)]
pub struct HouseGuildRightsMessage<'a> {
    #[protocol(var)]
    pub house_id: u32,
    pub instance_id: u32,
    pub second_hand: bool,
    pub guild_info: GuildInformations<'a>,
    #[protocol(var)]
    pub rights: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5704)]
pub struct HouseGuildShareRequestMessage<'a> {
    #[protocol(var)]
    pub house_id: u32,
    pub instance_id: u32,
    pub enable: bool,
    #[protocol(var)]
    pub rights: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5701)]
pub struct HouseGuildNoneMessage<'a> {
    #[protocol(var)]
    pub house_id: u32,
    pub instance_id: u32,
    pub second_hand: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5700)]
pub struct HouseGuildRightsViewMessage<'a> {
    #[protocol(var)]
    pub house_id: u32,
    pub instance_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
