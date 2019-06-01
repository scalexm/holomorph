pub mod anomaly;
pub mod breach;
pub mod death;
pub mod delay;
pub mod document;
pub mod emote;
pub mod fight;
pub mod havenbag;
pub mod houses;
pub mod job;
pub mod lockable;
pub mod npc;
pub mod objects;
pub mod paddock;
pub mod party;
pub mod purchasable;
pub mod quest;
pub mod spell;
pub mod stats;
pub mod treasure_hunt;
pub mod visual;

use crate::types::game::character::CharacterMinimalInformations;
use crate::types::game::context::fight::FightCommonInformations;
use crate::types::game::context::fight::FightExternalInformations;
use crate::types::game::context::fight::FightStartingPositions;
use crate::types::game::context::roleplay::party::NamedPartyTeam;
use crate::types::game::context::roleplay::AnomalySubareaInformation;
use crate::types::game::house::HouseInformationsInside;
use crate::types::game::interactive::MapObstacle;
use crate::types::game::interactive::StatedElement;
use crate::variants::GameFightFighterLightInformationsVariant;
use crate::variants::GameRolePlayActorInformationsVariant;
use crate::variants::HouseInformationsVariant;
use crate::variants::InteractiveElementVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 210)]
pub struct MapFightCountMessage<'a> {
    #[protocol(var)]
    pub fight_count: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6048)]
pub struct TeleportOnSameMapMessage<'a> {
    pub target_id: f64,
    #[protocol(var)]
    pub cell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 220)]
pub struct CurrentMapMessage<'a> {
    pub map_id: f64,
    pub map_key: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6622)]
pub struct MapComplementaryInformationsDataInHavenBagMessage<'a> {
    pub base: MapComplementaryInformationsDataMessage<'a>,
    pub owner_informations: CharacterMinimalInformations<'a>,
    pub theme: i8,
    pub room_id: u8,
    pub max_room_id: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5632)]
pub struct GameRolePlayShowActorMessage<'a> {
    pub informations: GameRolePlayActorInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6051)]
pub struct MapObstacleUpdateMessage<'a> {
    pub obstacles: std::borrow::Cow<'a, [MapObstacle<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6124)]
pub struct StopToListenRunningFightRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6738)]
pub struct CurrentMapInstanceMessage<'a> {
    pub base: CurrentMapMessage<'a>,
    pub instantiated_map_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6268)]
pub struct MapComplementaryInformationsWithCoordsMessage<'a> {
    pub base: MapComplementaryInformationsDataMessage<'a>,
    pub world_x: i16,
    pub world_y: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6500)]
pub struct MapRunningFightDetailsExtendedMessage<'a> {
    pub base: MapRunningFightDetailsMessage<'a>,
    pub named_party_teams: std::borrow::Cow<'a, [NamedPartyTeam<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 225)]
pub struct MapInformationsRequestMessage<'a> {
    pub map_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6407)]
pub struct GameRolePlayShowActorWithEventMessage<'a> {
    pub base: GameRolePlayShowActorMessage<'a>,
    pub actor_event_id: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6716)]
pub struct MapFightStartPositionsUpdateMessage<'a> {
    pub map_id: f64,
    pub fight_start_positions: FightStartingPositions<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6712)]
pub struct GameRolePlayShowMultipleActorsMessage<'a> {
    pub informations_list: std::borrow::Cow<'a, [GameRolePlayActorInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6130)]
pub struct MapComplementaryInformationsDataInHouseMessage<'a> {
    pub base: MapComplementaryInformationsDataMessage<'a>,
    pub current_house: HouseInformationsInside<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5743)]
pub struct MapRunningFightListMessage<'a> {
    pub fights: std::borrow::Cow<'a, [FightExternalInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5750)]
pub struct MapRunningFightDetailsRequestMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6827)]
pub struct MapRewardRateMessage<'a> {
    #[protocol(var)]
    pub map_rate: i16,
    #[protocol(var)]
    pub sub_area_rate: i16,
    #[protocol(var)]
    pub total_rate: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6835)]
pub struct AnomalySubareaInformationRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6197)]
pub struct ErrorMapNotFoundMessage<'a> {
    pub map_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5751)]
pub struct MapRunningFightDetailsMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub attackers: std::borrow::Cow<'a, [GameFightFighterLightInformationsVariant<'a>]>,
    pub defenders: std::borrow::Cow<'a, [GameFightFighterLightInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6836)]
pub struct AnomalySubareaInformationResponseMessage<'a> {
    pub subareas: std::borrow::Cow<'a, [AnomalySubareaInformation<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 221)]
pub struct ChangeMapMessage<'a> {
    pub map_id: f64,
    pub autopilot: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 226)]
pub struct MapComplementaryInformationsDataMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    pub map_id: f64,
    pub houses: std::borrow::Cow<'a, [HouseInformationsVariant<'a>]>,
    pub actors: std::borrow::Cow<'a, [GameRolePlayActorInformationsVariant<'a>]>,
    pub interactive_elements: std::borrow::Cow<'a, [InteractiveElementVariant<'a>]>,
    pub stated_elements: std::borrow::Cow<'a, [StatedElement<'a>]>,
    pub obstacles: std::borrow::Cow<'a, [MapObstacle<'a>]>,
    pub fights: std::borrow::Cow<'a, [FightCommonInformations<'a>]>,
    pub has_aggressive_monsters: bool,
    pub fight_start_positions: FightStartingPositions<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5742)]
pub struct MapRunningFightListRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
