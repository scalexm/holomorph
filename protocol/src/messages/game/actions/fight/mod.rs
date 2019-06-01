use crate::messages::game::actions::AbstractGameActionMessage;
use crate::types::game::actions::fight::GameActionMark;
use crate::types::game::look::EntityLook;
use crate::variants::AbstractFightDispellableEffectVariant;
use crate::variants::GameFightFighterInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6311)]
pub struct GameActionFightLifePointsGainMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    #[protocol(var)]
    pub delta: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6545)]
pub struct GameActionFightActivateGlyphTrapMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub mark_id: i16,
    pub active: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5531)]
pub struct GameActionFightReflectSpellMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5535)]
pub struct GameActionFightStealKamaMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    #[protocol(var)]
    pub amount: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5741)]
pub struct GameActionFightTriggerGlyphTrapMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub mark_id: i16,
    #[protocol(var)]
    pub mark_impact_cell: u16,
    pub triggering_character_id: f64,
    #[protocol(var)]
    pub triggered_spell_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6147)]
pub struct GameActionFightTriggerEffectMessage<'a> {
    pub base: GameActionFightDispellEffectMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6221)]
pub struct GameActionFightSpellImmunityMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    #[protocol(var)]
    pub spell_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5826)]
pub struct GameActionFightDropCharacterMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub cell_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6118)]
pub struct AbstractGameActionFightTargetedAbilityMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    #[protocol(flag)]
    pub silent_cast: bool,
    #[protocol(flag)]
    pub verbose_cast: bool,
    pub target_id: f64,
    pub destination_cell_id: i16,
    pub critical: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1099)]
pub struct GameActionFightDeathMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1005)]
pub struct GameActionFightCastRequestMessage<'a> {
    #[protocol(var)]
    pub spell_id: u16,
    pub cell_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5527)]
pub struct GameActionFightExchangePositionsMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub caster_cell_id: i16,
    pub target_cell_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6304)]
pub struct GameActionFightModifyEffectsDurationMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub delta: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6217)]
pub struct GameActionFightVanishMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6132)]
pub struct GameActionFightNoSpellCastMessage<'a> {
    #[protocol(var)]
    pub spell_level_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5533)]
pub struct GameActionFightDispellMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub verbose_cast: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5525)]
pub struct GameActionFightSlideMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub start_cell_id: i16,
    pub end_cell_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6310)]
pub struct GameActionFightLifeAndShieldPointsLostMessage<'a> {
    pub base: GameActionFightLifePointsLostMessage<'a>,
    #[protocol(var)]
    pub shield_loss: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6219)]
pub struct GameActionFightSpellCooldownVariationMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    #[protocol(var)]
    pub spell_id: u16,
    #[protocol(var)]
    pub value: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6113)]
pub struct GameActionFightDispellEffectMessage<'a> {
    pub base: GameActionFightDispellMessage<'a>,
    pub boost_uid: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5526)]
pub struct GameActionFightReduceDamagesMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    #[protocol(var)]
    pub amount: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6320)]
pub struct GameActionFightInvisibleDetectedMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub cell_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5528)]
pub struct GameActionFightTeleportOnSameMapMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub cell_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5530)]
pub struct GameActionFightReflectDamagesMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5540)]
pub struct GameActionFightMarkCellsMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub mark: GameActionMark<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5829)]
pub struct GameActionFightThrowCharacterMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub cell_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5821)]
pub struct GameActionFightInvisibilityMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub state: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5570)]
pub struct GameActionFightUnmarkCellsMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub mark_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6312)]
pub struct GameActionFightLifePointsLostMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    #[protocol(var)]
    pub loss: u32,
    #[protocol(var)]
    pub permanent_damages: u32,
    #[protocol(var)]
    pub element_id: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6176)]
pub struct GameActionFightDispellSpellMessage<'a> {
    pub base: GameActionFightDispellMessage<'a>,
    #[protocol(var)]
    pub spell_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5571)]
pub struct GameActionFightKillMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1004)]
pub struct GameActionFightTackledMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub tacklers_ids: std::borrow::Cow<'a, [f64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5828)]
pub struct GameActionFightDodgePointLossMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    #[protocol(var)]
    pub amount: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6116)]
pub struct GameActionFightCloseCombatMessage<'a> {
    pub base: AbstractGameActionFightTargetedAbilityMessage<'a>,
    #[protocol(var)]
    pub weapon_generic_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1010)]
pub struct GameActionFightSpellCastMessage<'a> {
    pub base: AbstractGameActionFightTargetedAbilityMessage<'a>,
    #[protocol(var)]
    pub spell_id: u16,
    pub spell_level: i16,
    pub portals_ids: std::borrow::Cow<'a, [i16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6070)]
pub struct GameActionFightDispellableEffectMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub effect: AbstractFightDispellableEffectVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5532)]
pub struct GameActionFightChangeLookMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub entity_look: EntityLook<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1030)]
pub struct GameActionFightPointsVariationMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub delta: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5830)]
pub struct GameActionFightCarryCharacterMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub target_id: f64,
    pub cell_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5825)]
pub struct GameActionFightSummonMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub summons: std::borrow::Cow<'a, [GameFightFighterInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6330)]
pub struct GameActionFightCastOnTargetRequestMessage<'a> {
    #[protocol(var)]
    pub spell_id: u16,
    pub target_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
