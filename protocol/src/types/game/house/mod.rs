use crate::types::game::context::roleplay::GuildInformations;
use crate::variants::HouseInstanceInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 512)]
pub struct HouseGuildedInformations<'a> {
    pub base: HouseInstanceInformations<'a>,
    pub guild_info: GuildInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 221)]
pub struct HouseInformationsForSell<'a> {
    pub instance_id: u32,
    pub second_hand: bool,
    #[protocol(var)]
    pub model_id: u32,
    pub owner_name: &'a str,
    pub owner_connected: bool,
    pub world_x: i16,
    pub world_y: i16,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub nb_room: i8,
    pub nb_chest: i8,
    pub skill_list_ids: std::borrow::Cow<'a, [i32]>,
    pub is_locked: bool,
    #[protocol(var)]
    pub price: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 390)]
pub struct AccountHouseInformations<'a> {
    pub base: HouseInformations<'a>,
    pub house_infos: HouseInstanceInformationsVariant<'a>,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 511)]
pub struct HouseInstanceInformations<'a> {
    #[protocol(flag)]
    pub second_hand: bool,
    #[protocol(flag)]
    pub is_locked: bool,
    #[protocol(flag)]
    pub is_sale_locked: bool,
    pub instance_id: u32,
    pub owner_name: &'a str,
    #[protocol(var)]
    pub price: i64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 510)]
pub struct HouseOnMapInformations<'a> {
    pub base: HouseInformations<'a>,
    pub doors_on_map: std::borrow::Cow<'a, [u32]>,
    pub house_instances: std::borrow::Cow<'a, [HouseInstanceInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 111)]
pub struct HouseInformations<'a> {
    #[protocol(var)]
    pub house_id: u32,
    #[protocol(var)]
    pub model_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 170)]
pub struct HouseInformationsForGuild<'a> {
    pub base: HouseInformations<'a>,
    pub instance_id: u32,
    pub second_hand: bool,
    pub owner_name: &'a str,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub skill_list_ids: std::borrow::Cow<'a, [i32]>,
    #[protocol(var)]
    pub guildshare_params: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 218)]
pub struct HouseInformationsInside<'a> {
    pub base: HouseInformations<'a>,
    pub house_infos: HouseInstanceInformationsVariant<'a>,
    pub world_x: i16,
    pub world_y: i16,
}
