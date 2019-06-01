use crate::types::game::data::items::effects::ObjectEffectInteger;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 178)]
pub struct MountClientData<'a> {
    #[protocol(flag)]
    pub sex: bool,
    #[protocol(flag)]
    pub is_rideable: bool,
    #[protocol(flag)]
    pub is_wild: bool,
    #[protocol(flag)]
    pub is_fecondation_ready: bool,
    #[protocol(flag)]
    pub use_harness_colors: bool,
    pub id: f64,
    #[protocol(var)]
    pub model: u32,
    pub ancestor: std::borrow::Cow<'a, [u32]>,
    pub behaviors: std::borrow::Cow<'a, [u32]>,
    pub name: &'a str,
    pub owner_id: u32,
    #[protocol(var)]
    pub experience: u64,
    #[protocol(var)]
    pub experience_for_level: u64,
    pub experience_for_next_level: f64,
    pub level: u8,
    #[protocol(var)]
    pub max_pods: u32,
    #[protocol(var)]
    pub stamina: u32,
    #[protocol(var)]
    pub stamina_max: u32,
    #[protocol(var)]
    pub maturity: u32,
    #[protocol(var)]
    pub maturity_for_adult: u32,
    #[protocol(var)]
    pub energy: u32,
    #[protocol(var)]
    pub energy_max: u32,
    pub serenity: i32,
    pub aggressivity_max: i32,
    #[protocol(var)]
    pub serenity_max: u32,
    #[protocol(var)]
    pub love: u32,
    #[protocol(var)]
    pub love_max: u32,
    pub fecondation_time: i32,
    pub boost_limiter: u32,
    pub boost_max: f64,
    pub reproduction_count: i32,
    #[protocol(var)]
    pub reproduction_count_max: u32,
    #[protocol(var)]
    pub harness_gid: u16,
    pub effect_list: std::borrow::Cow<'a, [ObjectEffectInteger<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 536)]
pub struct UpdateMountCharacteristic<'a> {
    pub type_: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 537)]
pub struct UpdateMountIntegerCharacteristic<'a> {
    pub base: UpdateMountCharacteristic<'a>,
    pub value: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 168)]
pub struct ItemDurability<'a> {
    pub durability: i16,
    pub durability_max: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 538)]
pub struct UpdateMountBooleanCharacteristic<'a> {
    pub base: UpdateMountCharacteristic<'a>,
    pub value: bool,
}
