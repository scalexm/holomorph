pub mod fight;
pub mod roleplay;

use crate::types::game::context::roleplay::AllianceInformations;
use crate::types::game::context::roleplay::GameRolePlayActorInformations;
use crate::types::game::context::roleplay::GuildInformations;
use crate::types::game::look::EntityLook;
use crate::variants::EntityDispositionInformationsVariant;
use crate::variants::TaxCollectorStaticInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 147)]
pub struct TaxCollectorStaticInformations<'a> {
    #[protocol(var)]
    pub first_name_id: u16,
    #[protocol(var)]
    pub last_name_id: u16,
    pub guild_identity: GuildInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 217)]
pub struct FightEntityDispositionInformations<'a> {
    pub base: EntityDispositionInformations<'a>,
    pub carrying_character_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 353)]
pub struct ActorOrientation<'a> {
    pub id: f64,
    pub direction: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 60)]
pub struct EntityDispositionInformations<'a> {
    pub cell_id: i16,
    pub direction: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 148)]
pub struct GameRolePlayTaxCollectorInformations<'a> {
    pub base: GameRolePlayActorInformations<'a>,
    pub identification: TaxCollectorStaticInformationsVariant<'a>,
    pub guild_level: u8,
    pub tax_collector_attack: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 150)]
pub struct GameContextActorInformations<'a> {
    pub contextual_id: f64,
    pub look: EntityLook<'a>,
    pub disposition: EntityDispositionInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 107)]
pub struct IdentifiedEntityDispositionInformations<'a> {
    pub base: EntityDispositionInformations<'a>,
    pub id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 176)]
pub struct MapCoordinatesExtended<'a> {
    pub base: MapCoordinatesAndId<'a>,
    #[protocol(var)]
    pub sub_area_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 63)]
pub struct EntityMovementInformations<'a> {
    pub id: i32,
    pub steps: &'a [i8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 174)]
pub struct MapCoordinates<'a> {
    pub world_x: i16,
    pub world_y: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 440)]
pub struct TaxCollectorStaticExtendedInformations<'a> {
    pub base: TaxCollectorStaticInformations<'a>,
    pub alliance_identity: AllianceInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 392)]
pub struct MapCoordinatesAndId<'a> {
    pub base: MapCoordinates<'a>,
    pub map_id: f64,
}
