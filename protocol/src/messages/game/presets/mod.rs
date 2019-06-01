use crate::types::game::presets::ItemForPreset;
use crate::variants::PresetVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6755)]
pub struct PresetDeleteRequestMessage<'a> {
    pub preset_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6760)]
pub struct ItemForPresetUpdateMessage<'a> {
    pub preset_id: i16,
    pub preset_item: ItemForPreset<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6762)]
pub struct PresetSaveErrorMessage<'a> {
    pub preset_id: i16,
    pub code: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6756)]
pub struct CharacterPresetSaveRequestMessage<'a> {
    pub base: PresetSaveRequestMessage<'a>,
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6747)]
pub struct PresetUseResultMessage<'a> {
    pub preset_id: i16,
    pub code: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6757)]
pub struct PresetUseResultWithMissingIdsMessage<'a> {
    pub base: PresetUseResultMessage<'a>,
    #[protocol(var_contents)]
    pub missing_ids: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6758)]
pub struct IdolsPresetSaveRequestMessage<'a> {
    pub base: PresetSaveRequestMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6761)]
pub struct PresetSaveRequestMessage<'a> {
    pub preset_id: i16,
    pub symbol_id: u8,
    pub update_data: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6763)]
pub struct PresetSavedMessage<'a> {
    pub preset_id: i16,
    pub preset: PresetVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6748)]
pub struct PresetDeleteResultMessage<'a> {
    pub preset_id: i16,
    pub code: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6759)]
pub struct PresetUseRequestMessage<'a> {
    pub preset_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6750)]
pub struct PresetsMessage<'a> {
    pub presets: std::borrow::Cow<'a, [PresetVariant<'a>]>,
}
