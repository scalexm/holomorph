pub mod display;
pub mod dungeon;
pub mod fight;
pub mod mount;
pub mod notification;
pub mod roleplay;

use crate::types::game::context::roleplay::MonsterBoosts;
use crate::types::game::context::ActorOrientation;
use crate::types::game::context::EntityMovementInformations;
use crate::types::game::context::IdentifiedEntityDispositionInformations;
use crate::types::game::look::EntityLook;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 253)]
pub struct GameContextMoveElementMessage<'a> {
    pub movement: EntityMovementInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5693)]
pub struct GameEntityDispositionMessage<'a> {
    pub disposition: IdentifiedEntityDispositionInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 945)]
pub struct GameMapChangeOrientationRequestMessage<'a> {
    pub direction: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6416)]
pub struct GameContextRemoveMultipleElementsWithEventsMessage<'a> {
    pub base: GameContextRemoveMultipleElementsMessage<'a>,
    pub element_event_ids: &'a [u8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 255)]
pub struct GameContextQuitMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 200)]
pub struct GameContextCreateMessage<'a> {
    pub context: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5637)]
pub struct GameContextRefreshEntityLookMessage<'a> {
    pub id: f64,
    pub look: EntityLook<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 254)]
pub struct GameContextMoveMultipleElementsMessage<'a> {
    pub movements: std::borrow::Cow<'a, [EntityMovementInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 950)]
pub struct GameMapMovementRequestMessage<'a> {
    pub key_movements: std::borrow::Cow<'a, [u16]>,
    pub map_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6155)]
pub struct GameMapChangeOrientationsMessage<'a> {
    pub orientations: std::borrow::Cow<'a, [ActorOrientation<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6412)]
pub struct GameContextRemoveElementWithEventMessage<'a> {
    pub base: GameContextRemoveElementMessage<'a>,
    pub element_event_id: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6024)]
pub struct GameContextCreateErrorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 201)]
pub struct GameContextDestroyMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 952)]
pub struct GameMapMovementConfirmMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 250)]
pub struct GameContextCreateRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6497)]
pub struct GameCautiousMapMovementMessage<'a> {
    pub base: GameMapMovementMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6081)]
pub struct GameContextKickMessage<'a> {
    pub target_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 251)]
pub struct GameContextRemoveElementMessage<'a> {
    pub id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5611)]
pub struct ShowCellRequestMessage<'a> {
    #[protocol(var)]
    pub cell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 953)]
pub struct GameMapMovementCancelMessage<'a> {
    #[protocol(var)]
    pub cell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6496)]
pub struct GameCautiousMapMovementRequestMessage<'a> {
    pub base: GameMapMovementRequestMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5612)]
pub struct ShowCellMessage<'a> {
    pub source_id: f64,
    #[protocol(var)]
    pub cell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 252)]
pub struct GameContextRemoveMultipleElementsMessage<'a> {
    pub elements_ids: std::borrow::Cow<'a, [f64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5696)]
pub struct GameEntitiesDispositionMessage<'a> {
    pub dispositions: std::borrow::Cow<'a, [IdentifiedEntityDispositionInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5695)]
pub struct GameEntityDispositionErrorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6158)]
pub struct ShowCellSpectatorMessage<'a> {
    pub base: ShowCellMessage<'a>,
    pub player_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 946)]
pub struct GameMapChangeOrientationMessage<'a> {
    pub orientation: ActorOrientation<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 954)]
pub struct GameMapNoMovementMessage<'a> {
    pub cell_x: i16,
    pub cell_y: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6618)]
pub struct GameRefreshMonsterBoostsMessage<'a> {
    pub monster_boosts: std::borrow::Cow<'a, [MonsterBoosts<'a>]>,
    pub family_boosts: std::borrow::Cow<'a, [MonsterBoosts<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 951)]
pub struct GameMapMovementMessage<'a> {
    pub key_movements: std::borrow::Cow<'a, [u16]>,
    pub forced_direction: i16,
    pub actor_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6071)]
pub struct GameContextReadyMessage<'a> {
    pub map_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
