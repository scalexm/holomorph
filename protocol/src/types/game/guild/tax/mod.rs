use crate::types::game::context::roleplay::BasicGuildInformations;
use crate::types::game::fight::ProtectedEntityWaitingForHelpInfo;
use crate::types::game::look::EntityLook;
use crate::variants::CharacterMinimalPlusLookInformationsVariant;
use crate::variants::TaxCollectorComplementaryInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 165)]
pub struct AdditionalTaxCollectorInformations<'a> {
    pub collector_caller_name: &'a str,
    pub date: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 493)]
pub struct TaxCollectorMovement<'a> {
    pub movement_type: u8,
    pub basic_infos: TaxCollectorBasicInformations<'a>,
    #[protocol(var)]
    pub player_id: u64,
    pub player_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 169)]
pub struct TaxCollectorFightersInformation<'a> {
    pub collector_id: f64,
    pub ally_characters_informations:
        std::borrow::Cow<'a, [CharacterMinimalPlusLookInformationsVariant<'a>]>,
    pub enemy_characters_informations:
        std::borrow::Cow<'a, [CharacterMinimalPlusLookInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 448)]
pub struct TaxCollectorComplementaryInformations<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 372)]
pub struct TaxCollectorLootInformations<'a> {
    pub base: TaxCollectorComplementaryInformations<'a>,
    #[protocol(var)]
    pub kamas: u64,
    #[protocol(var)]
    pub experience: u64,
    #[protocol(var)]
    pub pods: u32,
    #[protocol(var)]
    pub items_value: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 167)]
pub struct TaxCollectorInformations<'a> {
    pub unique_id: f64,
    #[protocol(var)]
    pub firt_name_id: u16,
    #[protocol(var)]
    pub last_name_id: u16,
    pub additional_infos: AdditionalTaxCollectorInformations<'a>,
    pub world_x: i16,
    pub world_y: i16,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub state: u8,
    pub look: EntityLook<'a>,
    pub complements: std::borrow::Cow<'a, [TaxCollectorComplementaryInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 447)]
pub struct TaxCollectorWaitingForHelpInformations<'a> {
    pub base: TaxCollectorComplementaryInformations<'a>,
    pub waiting_for_help_info: ProtectedEntityWaitingForHelpInfo<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 446)]
pub struct TaxCollectorGuildInformations<'a> {
    pub base: TaxCollectorComplementaryInformations<'a>,
    pub guild: BasicGuildInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 96)]
pub struct TaxCollectorBasicInformations<'a> {
    #[protocol(var)]
    pub first_name_id: u16,
    #[protocol(var)]
    pub last_name_id: u16,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
