use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 74)]
pub struct ObjectEffectString<'a> {
    pub base: ObjectEffect<'a>,
    pub value: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 75)]
pub struct ObjectEffectDuration<'a> {
    pub base: ObjectEffect<'a>,
    #[protocol(var)]
    pub days: u16,
    pub hours: u8,
    pub minutes: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 72)]
pub struct ObjectEffectDate<'a> {
    pub base: ObjectEffect<'a>,
    #[protocol(var)]
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 81)]
pub struct ObjectEffectLadder<'a> {
    pub base: ObjectEffectCreature<'a>,
    #[protocol(var)]
    pub monster_count: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 82)]
pub struct ObjectEffectMinMax<'a> {
    pub base: ObjectEffect<'a>,
    #[protocol(var)]
    pub min: u32,
    #[protocol(var)]
    pub max: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 71)]
pub struct ObjectEffectCreature<'a> {
    pub base: ObjectEffect<'a>,
    #[protocol(var)]
    pub monster_family_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 73)]
pub struct ObjectEffectDice<'a> {
    pub base: ObjectEffect<'a>,
    #[protocol(var)]
    pub dice_num: u32,
    #[protocol(var)]
    pub dice_side: u32,
    #[protocol(var)]
    pub dice_const: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 70)]
pub struct ObjectEffectInteger<'a> {
    pub base: ObjectEffect<'a>,
    #[protocol(var)]
    pub value: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 76)]
pub struct ObjectEffect<'a> {
    #[protocol(var)]
    pub action_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 179)]
pub struct ObjectEffectMount<'a> {
    pub base: ObjectEffect<'a>,
    #[protocol(flag)]
    pub sex: bool,
    #[protocol(flag)]
    pub is_rideable: bool,
    #[protocol(flag)]
    pub is_feconded: bool,
    #[protocol(flag)]
    pub is_fecondation_ready: bool,
    #[protocol(var)]
    pub id: u64,
    #[protocol(var)]
    pub expiration_date: u64,
    #[protocol(var)]
    pub model: u32,
    pub name: &'a str,
    pub owner: &'a str,
    pub level: u8,
    #[protocol(var)]
    pub reproduction_count: i32,
    #[protocol(var)]
    pub reproduction_count_max: u32,
    pub effects: std::borrow::Cow<'a, [ObjectEffectInteger<'a>]>,
    #[protocol(var_contents)]
    pub capacities: std::borrow::Cow<'a, [u32]>,
}
