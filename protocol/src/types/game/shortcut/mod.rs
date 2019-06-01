use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 371)]
pub struct ShortcutObjectItem<'a> {
    pub base: ShortcutObject<'a>,
    pub item_uid: i32,
    pub item_gid: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 370)]
pub struct ShortcutObjectPreset<'a> {
    pub base: ShortcutObject<'a>,
    pub preset_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 367)]
pub struct ShortcutObject<'a> {
    pub base: Shortcut<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 369)]
pub struct Shortcut<'a> {
    pub slot: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 389)]
pub struct ShortcutEmote<'a> {
    pub base: Shortcut<'a>,
    pub emote_id: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 388)]
pub struct ShortcutSmiley<'a> {
    pub base: Shortcut<'a>,
    #[protocol(var)]
    pub smiley_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 368)]
pub struct ShortcutSpell<'a> {
    pub base: Shortcut<'a>,
    #[protocol(var)]
    pub spell_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 492)]
pub struct ShortcutObjectIdolsPreset<'a> {
    pub base: ShortcutObject<'a>,
    pub preset_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 544)]
pub struct ShortcutEntitiesPreset<'a> {
    pub base: Shortcut<'a>,
    pub preset_id: i16,
}
