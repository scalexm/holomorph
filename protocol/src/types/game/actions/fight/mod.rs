use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 207)]
pub struct FightTemporarySpellBoostEffect<'a> {
    pub base: FightTemporaryBoostEffect<'a>,
    #[protocol(var)]
    pub boosted_spell_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 366)]
pub struct FightTemporarySpellImmunityEffect<'a> {
    pub base: AbstractFightDispellableEffect<'a>,
    pub immune_spell_id: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 206)]
pub struct AbstractFightDispellableEffect<'a> {
    #[protocol(var)]
    pub uid: u32,
    pub target_id: f64,
    pub turn_duration: i16,
    pub dispelable: u8,
    #[protocol(var)]
    pub spell_id: u16,
    #[protocol(var)]
    pub effect_id: u32,
    #[protocol(var)]
    pub parent_boost_uid: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 211)]
pub struct FightTemporaryBoostWeaponDamagesEffect<'a> {
    pub base: FightTemporaryBoostEffect<'a>,
    pub weapon_type_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 210)]
pub struct FightTriggeredEffect<'a> {
    pub base: AbstractFightDispellableEffect<'a>,
    pub param1: i32,
    pub param2: i32,
    pub param3: i32,
    pub delay: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 85)]
pub struct GameActionMarkedCell<'a> {
    #[protocol(var)]
    pub cell_id: u16,
    pub zone_size: i8,
    pub cell_color: i32,
    pub cells_type: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 351)]
pub struct GameActionMark<'a> {
    pub mark_author_id: f64,
    pub mark_team_id: u8,
    pub mark_spell_id: u32,
    pub mark_spell_level: i16,
    pub mark_id: i16,
    pub mark_type: i8,
    pub markimpact_cell: i16,
    pub cells: std::borrow::Cow<'a, [GameActionMarkedCell<'a>]>,
    pub active: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 209)]
pub struct FightTemporaryBoostEffect<'a> {
    pub base: AbstractFightDispellableEffect<'a>,
    pub delta: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 214)]
pub struct FightTemporaryBoostStateEffect<'a> {
    pub base: FightTemporaryBoostEffect<'a>,
    pub state_id: i16,
}
