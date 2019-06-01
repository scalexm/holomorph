use crate::types::game::look::EntityLook;
use crate::variants::PresetVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 539)]
pub struct CharacterCharacteristicForPreset<'a> {
    pub base: SimpleCharacterCharacteristicForPreset<'a>,
    #[protocol(var)]
    pub stuff: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 517)]
pub struct ItemsPreset<'a> {
    pub base: Preset<'a>,
    pub items: std::borrow::Cow<'a, [ItemForPreset<'a>]>,
    pub mount_equipped: bool,
    pub look: EntityLook<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 520)]
pub struct PresetsContainerPreset<'a> {
    pub base: Preset<'a>,
    pub presets: std::borrow::Cow<'a, [PresetVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 532)]
pub struct FullStatsPreset<'a> {
    pub base: Preset<'a>,
    pub stats: std::borrow::Cow<'a, [CharacterCharacteristicForPreset<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 521)]
pub struct StatsPreset<'a> {
    pub base: Preset<'a>,
    pub stats: std::borrow::Cow<'a, [SimpleCharacterCharacteristicForPreset<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 519)]
pub struct SpellsPreset<'a> {
    pub base: Preset<'a>,
    pub spells: std::borrow::Cow<'a, [SpellForPreset<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 540)]
pub struct ItemForPreset<'a> {
    pub position: u16,
    #[protocol(var)]
    pub obj_gid: u16,
    #[protocol(var)]
    pub obj_uid: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 491)]
pub struct IdolsPreset<'a> {
    pub base: Preset<'a>,
    pub icon_id: u16,
    #[protocol(var_contents)]
    pub idol_ids: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 541)]
pub struct SimpleCharacterCharacteristicForPreset<'a> {
    pub keyword: &'a str,
    #[protocol(var)]
    pub base: i16,
    #[protocol(var)]
    pub additionnal: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 534)]
pub struct CharacterBuildPreset<'a> {
    pub base: PresetsContainerPreset<'a>,
    pub icon_id: u16,
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 545)]
pub struct EntitiesPreset<'a> {
    pub base: Preset<'a>,
    pub icon_id: u16,
    #[protocol(var_contents)]
    pub entity_ids: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 355)]
pub struct Preset<'a> {
    pub id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 557)]
pub struct SpellForPreset<'a> {
    #[protocol(var)]
    pub spell_id: u16,
    pub shortcuts: std::borrow::Cow<'a, [i16]>,
}
