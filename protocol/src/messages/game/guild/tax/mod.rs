use crate::types::game::character::CharacterMinimalPlusLookInformations;
use crate::types::game::context::roleplay::BasicGuildInformations;
use crate::types::game::guild::tax::TaxCollectorBasicInformations;
use crate::types::game::guild::tax::TaxCollectorFightersInformation;
use crate::types::game::guild::tax::TaxCollectorMovement;
use crate::variants::TaxCollectorInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5930)]
pub struct TaxCollectorListMessage<'a> {
    pub base: AbstractTaxCollectorListMessage<'a>,
    pub nbcollector_max: u8,
    pub fighters_informations: std::borrow::Cow<'a, [TaxCollectorFightersInformation<'a>]>,
    pub info_type: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5634)]
pub struct TaxCollectorErrorMessage<'a> {
    pub reason: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5928)]
pub struct GuildFightPlayersEnemiesListMessage<'a> {
    pub fight_id: f64,
    pub player_info: std::borrow::Cow<'a, [CharacterMinimalPlusLookInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5720)]
pub struct GuildFightPlayersHelpersJoinMessage<'a> {
    pub fight_id: f64,
    pub player_info: CharacterMinimalPlusLookInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5915)]
pub struct TaxCollectorMovementRemoveMessage<'a> {
    pub collector_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5633)]
pub struct TaxCollectorMovementMessage<'a> {
    pub movement_type: u8,
    pub basic_infos: TaxCollectorBasicInformations<'a>,
    #[protocol(var)]
    pub player_id: u64,
    pub player_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5717)]
pub struct GuildFightJoinRequestMessage<'a> {
    pub tax_collector_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6565)]
pub struct TopTaxCollectorListMessage<'a> {
    pub base: AbstractTaxCollectorListMessage<'a>,
    pub is_dungeon: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5719)]
pub struct GuildFightPlayersHelpersLeaveMessage<'a> {
    pub fight_id: f64,
    #[protocol(var)]
    pub player_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5954)]
pub struct GameRolePlayTaxCollectorFightRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5917)]
pub struct TaxCollectorMovementAddMessage<'a> {
    pub informations: TaxCollectorInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5635)]
pub struct TaxCollectorAttackedResultMessage<'a> {
    pub dead_or_alive: bool,
    pub basic_infos: TaxCollectorBasicInformations<'a>,
    pub guild: BasicGuildInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6455)]
pub struct TaxCollectorStateUpdateMessage<'a> {
    pub unique_id: f64,
    pub state: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6235)]
pub struct GuildFightTakePlaceRequestMessage<'a> {
    pub base: GuildFightJoinRequestMessage<'a>,
    pub replaced_character_id: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5929)]
pub struct GuildFightPlayersEnemyRemoveMessage<'a> {
    pub fight_id: f64,
    #[protocol(var)]
    pub player_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5918)]
pub struct TaxCollectorAttackedMessage<'a> {
    #[protocol(var)]
    pub first_name_id: u16,
    #[protocol(var)]
    pub last_name_id: u16,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub guild: BasicGuildInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5715)]
pub struct GuildFightLeaveRequestMessage<'a> {
    pub tax_collector_id: f64,
    #[protocol(var)]
    pub character_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6611)]
pub struct TaxCollectorMovementsOfflineMessage<'a> {
    pub movements: std::borrow::Cow<'a, [TaxCollectorMovement<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6568)]
pub struct AbstractTaxCollectorListMessage<'a> {
    pub informations: std::borrow::Cow<'a, [TaxCollectorInformationsVariant<'a>]>,
}
