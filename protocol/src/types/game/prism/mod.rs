use crate::types::game::context::roleplay::AllianceInformations;
use crate::types::game::data::items::ObjectItem;
use crate::types::game::fight::ProtectedEntityWaitingForHelpInfo;
use crate::variants::CharacterMinimalPlusLookInformationsVariant;
use crate::variants::PrismInformationVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 431)]
pub struct AllianceInsiderPrismInformation<'a> {
    pub base: PrismInformation<'a>,
    pub last_time_slot_modification_date: u32,
    #[protocol(var)]
    pub last_time_slot_modification_author_guild_id: u32,
    #[protocol(var)]
    pub last_time_slot_modification_author_id: u64,
    pub last_time_slot_modification_author_name: &'a str,
    pub modules_objects: std::borrow::Cow<'a, [ObjectItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 438)]
pub struct PrismSubareaEmptyInfo<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    #[protocol(var)]
    pub alliance_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 434)]
pub struct PrismGeolocalizedInformation<'a> {
    pub base: PrismSubareaEmptyInfo<'a>,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    pub prism: PrismInformationVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 443)]
pub struct PrismFightersInformation<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    pub waiting_for_help_info: ProtectedEntityWaitingForHelpInfo<'a>,
    pub ally_characters_informations:
        std::borrow::Cow<'a, [CharacterMinimalPlusLookInformationsVariant<'a>]>,
    pub enemy_characters_informations:
        std::borrow::Cow<'a, [CharacterMinimalPlusLookInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 427)]
pub struct AlliancePrismInformation<'a> {
    pub base: PrismInformation<'a>,
    pub alliance: AllianceInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 428)]
pub struct PrismInformation<'a> {
    pub type_id: u8,
    pub state: u8,
    pub next_vulnerability_date: u32,
    pub placement_date: u32,
    #[protocol(var)]
    pub reward_token_count: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
