pub mod effects;

use crate::variants::ObjectEffectVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 134)]
pub struct ObjectItemNotInContainer<'a> {
    pub base: Item<'a>,
    #[protocol(var)]
    pub object_gid: u16,
    pub effects: std::borrow::Cow<'a, [ObjectEffectVariant<'a>]>,
    #[protocol(var)]
    pub object_uid: u32,
    #[protocol(var)]
    pub quantity: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 387)]
pub struct ObjectItemInformationWithQuantity<'a> {
    pub base: ObjectItemMinimalInformation<'a>,
    #[protocol(var)]
    pub quantity: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 37)]
pub struct ObjectItem<'a> {
    pub base: Item<'a>,
    pub position: u16,
    #[protocol(var)]
    pub object_gid: u16,
    pub effects: std::borrow::Cow<'a, [ObjectEffectVariant<'a>]>,
    #[protocol(var)]
    pub object_uid: u32,
    #[protocol(var)]
    pub quantity: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 120)]
pub struct ObjectItemToSell<'a> {
    pub base: Item<'a>,
    #[protocol(var)]
    pub object_gid: u16,
    pub effects: std::borrow::Cow<'a, [ObjectEffectVariant<'a>]>,
    #[protocol(var)]
    pub object_uid: u32,
    #[protocol(var)]
    pub quantity: u32,
    #[protocol(var)]
    pub object_price: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 164)]
pub struct ObjectItemToSellInBid<'a> {
    pub base: ObjectItemToSell<'a>,
    pub unsold_delay: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 119)]
pub struct ObjectItemQuantity<'a> {
    pub base: Item<'a>,
    #[protocol(var)]
    pub object_uid: u32,
    #[protocol(var)]
    pub quantity: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 7)]
pub struct Item<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 483)]
pub struct ObjectItemGenericQuantity<'a> {
    pub base: Item<'a>,
    #[protocol(var)]
    pub object_gid: u16,
    #[protocol(var)]
    pub quantity: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 123)]
pub struct GoldItem<'a> {
    pub base: Item<'a>,
    #[protocol(var)]
    pub sum: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 359)]
pub struct ObjectItemToSellInHumanVendorShop<'a> {
    pub base: Item<'a>,
    #[protocol(var)]
    pub object_gid: u16,
    pub effects: std::borrow::Cow<'a, [ObjectEffectVariant<'a>]>,
    #[protocol(var)]
    pub object_uid: u32,
    #[protocol(var)]
    pub quantity: u32,
    #[protocol(var)]
    pub object_price: u64,
    #[protocol(var)]
    pub public_price: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 121)]
pub struct SellerBuyerDescriptor<'a> {
    #[protocol(var_contents)]
    pub quantities: std::borrow::Cow<'a, [u32]>,
    #[protocol(var_contents)]
    pub types: std::borrow::Cow<'a, [u32]>,
    pub tax_percentage: f32,
    pub tax_modification_percentage: f32,
    pub max_item_level: u8,
    #[protocol(var)]
    pub max_item_per_account: u32,
    pub npc_contextual_id: i32,
    #[protocol(var)]
    pub unsold_delay: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 49)]
pub struct SpellItem<'a> {
    pub base: Item<'a>,
    pub spell_id: i32,
    pub spell_level: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 122)]
pub struct BidExchangerObjectInfo<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    pub effects: std::borrow::Cow<'a, [ObjectEffectVariant<'a>]>,
    #[protocol(var_contents)]
    pub prices: std::borrow::Cow<'a, [u64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 352)]
pub struct ObjectItemToSellInNpcShop<'a> {
    pub base: ObjectItemMinimalInformation<'a>,
    #[protocol(var)]
    pub object_price: u64,
    pub buy_criterion: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 124)]
pub struct ObjectItemMinimalInformation<'a> {
    pub base: Item<'a>,
    #[protocol(var)]
    pub object_gid: u16,
    pub effects: std::borrow::Cow<'a, [ObjectEffectVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 494)]
pub struct ObjectItemGenericQuantityPrice<'a> {
    pub base: ObjectItemGenericQuantity<'a>,
    #[protocol(var)]
    pub price: u64,
}
