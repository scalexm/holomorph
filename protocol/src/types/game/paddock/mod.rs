use crate::types::game::context::roleplay::GuildInformations;
use crate::types::game::context::roleplay::ObjectItemInRolePlay;
use crate::types::game::mount::ItemDurability;
use crate::variants::PaddockBuyableInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 185)]
pub struct PaddockItem<'a> {
    pub base: ObjectItemInRolePlay<'a>,
    pub durability: ItemDurability<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 509)]
pub struct PaddockInstancesInformations<'a> {
    pub base: PaddockInformations<'a>,
    pub paddocks: std::borrow::Cow<'a, [PaddockBuyableInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 222)]
pub struct PaddockInformationsForSell<'a> {
    pub guild_owner: &'a str,
    pub world_x: i16,
    pub world_y: i16,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub nb_mount: i8,
    pub nb_object: i8,
    #[protocol(var)]
    pub price: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 508)]
pub struct PaddockGuildedInformations<'a> {
    pub base: PaddockBuyableInformations<'a>,
    pub deserted: bool,
    pub guild_info: GuildInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 132)]
pub struct PaddockInformations<'a> {
    #[protocol(var)]
    pub max_outdoor_mount: u16,
    #[protocol(var)]
    pub max_items: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 184)]
pub struct MountInformationsForPaddock<'a> {
    #[protocol(var)]
    pub model_id: u16,
    pub name: &'a str,
    pub owner_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 183)]
pub struct PaddockContentInformations<'a> {
    pub base: PaddockInformations<'a>,
    pub paddock_id: f64,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub abandonned: bool,
    pub mounts_informations: std::borrow::Cow<'a, [MountInformationsForPaddock<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 130)]
pub struct PaddockBuyableInformations<'a> {
    #[protocol(var)]
    pub price: u64,
    pub locked: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
