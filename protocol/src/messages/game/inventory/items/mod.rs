use crate::messages::game::inventory::exchanges::ExchangeObjectMessage;
use crate::types::game::data::items::GoldItem;
use crate::types::game::data::items::ObjectItem;
use crate::types::game::data::items::ObjectItemQuantity;
use crate::variants::ObjectEffectVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5521)]
pub struct ExchangeKamaModifiedMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    #[protocol(var)]
    pub quantity: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6020)]
pub struct ExchangeMultiCraftCrafterCanUseHisRessourcesMessage<'a> {
    pub allowed: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3010)]
pub struct ObjectMovementMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    pub position: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3021)]
pub struct ObjectSetPositionMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    pub position: u16,
    #[protocol(var)]
    pub quantity: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5503)]
pub struct SetUpdateMessage<'a> {
    #[protocol(var)]
    pub set_id: u16,
    #[protocol(var_contents)]
    pub set_objects: std::borrow::Cow<'a, [u16]>,
    pub set_effects: std::borrow::Cow<'a, [ObjectEffectVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6670)]
pub struct ExchangePodsModifiedMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    #[protocol(var)]
    pub current_weight: u32,
    #[protocol(var)]
    pub max_weight: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5725)]
pub struct LivingObjectChangeSkinRequestMessage<'a> {
    #[protocol(var)]
    pub living_uid: u32,
    pub living_position: u8,
    #[protocol(var)]
    pub skin_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6523)]
pub struct WrapperObjectAssociatedMessage<'a> {
    pub base: SymbioticObjectAssociatedMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6034)]
pub struct ObjectsDeletedMessage<'a> {
    #[protocol(var_contents)]
    pub object_uid: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3029)]
pub struct ObjectModifiedMessage<'a> {
    pub object: ObjectItem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6014)]
pub struct ObjectJobAddedMessage<'a> {
    pub job_id: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6066)]
pub struct LivingObjectMessageRequestMessage<'a> {
    #[protocol(var)]
    pub msg_id: u16,
    pub parameters: std::borrow::Cow<'a, [&'a str]>,
    #[protocol(var)]
    pub living_object: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3025)]
pub struct ObjectAddedMessage<'a> {
    pub object: ObjectItem<'a>,
    pub origin: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6529)]
pub struct WrapperObjectErrorMessage<'a> {
    pub base: SymbioticObjectErrorMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6010)]
pub struct ExchangeObjectRemovedFromBagMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    #[protocol(var)]
    pub object_uid: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6460)]
pub struct MimicryObjectFeedAndAssociateRequestMessage<'a> {
    pub base: SymbioticObjectAssociateRequestMessage<'a>,
    #[protocol(var)]
    pub food_uid: u32,
    pub food_pos: u8,
    pub preview: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5519)]
pub struct ExchangeObjectModifiedMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    pub object: ObjectItem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3004)]
pub struct ObjectErrorMessage<'a> {
    pub reason: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6458)]
pub struct MimicryObjectPreviewMessage<'a> {
    pub result: ObjectItem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3023)]
pub struct ObjectQuantityMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    #[protocol(var)]
    pub quantity: u32,
    pub origin: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5723)]
pub struct LivingObjectDissociateMessage<'a> {
    #[protocol(var)]
    pub living_uid: u32,
    pub living_position: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6009)]
pub struct ExchangeObjectPutInBagMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    pub object: ObjectItem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3022)]
pub struct ObjectDeleteMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    #[protocol(var)]
    pub quantity: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3019)]
pub struct ObjectUseMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6526)]
pub struct SymbioticObjectErrorMessage<'a> {
    pub base: ObjectErrorMessage<'a>,
    pub error_code: i8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3024)]
pub struct ObjectDeletedMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6522)]
pub struct SymbioticObjectAssociateRequestMessage<'a> {
    #[protocol(var)]
    pub symbiote_uid: u32,
    pub symbiote_pos: u8,
    #[protocol(var)]
    pub host_uid: u32,
    pub host_pos: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6527)]
pub struct SymbioticObjectAssociatedMessage<'a> {
    #[protocol(var)]
    pub host_uid: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3013)]
pub struct ObjectUseOnCellMessage<'a> {
    pub base: ObjectUseMessage<'a>,
    #[protocol(var)]
    pub cells: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3009)]
pub struct InventoryWeightMessage<'a> {
    #[protocol(var)]
    pub weight: u32,
    #[protocol(var)]
    pub weight_max: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6030)]
pub struct GoldAddedMessage<'a> {
    pub gold: GoldItem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6532)]
pub struct ExchangeObjectsRemovedMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    #[protocol(var_contents)]
    pub object_uid: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5517)]
pub struct ExchangeObjectRemovedMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    #[protocol(var)]
    pub object_uid: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6524)]
pub struct WrapperObjectDissociateRequestMessage<'a> {
    #[protocol(var)]
    pub host_uid: u32,
    pub host_pos: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6234)]
pub struct ObjectUseMultipleMessage<'a> {
    pub base: ObjectUseMessage<'a>,
    #[protocol(var)]
    pub quantity: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6065)]
pub struct LivingObjectMessageMessage<'a> {
    #[protocol(var)]
    pub msg_id: u16,
    pub time_stamp: u32,
    pub owner: &'a str,
    #[protocol(var)]
    pub object_generic_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6206)]
pub struct ObjectsQuantityMessage<'a> {
    pub objects_uid_and_qty: std::borrow::Cow<'a, [ObjectItemQuantity<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6457)]
pub struct MimicryObjectEraseRequestMessage<'a> {
    #[protocol(var)]
    pub host_uid: u32,
    pub host_pos: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6519)]
pub struct ObtainedItemMessage<'a> {
    #[protocol(var)]
    pub generic_id: u16,
    #[protocol(var)]
    pub base_quantity: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6520)]
pub struct ObtainedItemWithBonusMessage<'a> {
    pub base: ObtainedItemMessage<'a>,
    #[protocol(var)]
    pub bonus_quantity: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3016)]
pub struct InventoryContentMessage<'a> {
    pub objects: std::borrow::Cow<'a, [ObjectItem<'a>]>,
    #[protocol(var)]
    pub kamas: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6021)]
pub struct ExchangeMultiCraftSetCrafterCanUseHisRessourcesMessage<'a> {
    pub allow: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6462)]
pub struct MimicryObjectAssociatedMessage<'a> {
    pub base: SymbioticObjectAssociatedMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3005)]
pub struct ObjectDropMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    #[protocol(var)]
    pub quantity: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6533)]
pub struct ExchangeObjectsModifiedMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    pub object: std::borrow::Cow<'a, [ObjectItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6461)]
pub struct MimicryObjectErrorMessage<'a> {
    pub base: SymbioticObjectErrorMessage<'a>,
    pub preview: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3003)]
pub struct ObjectUseOnCharacterMessage<'a> {
    pub base: ObjectUseMessage<'a>,
    #[protocol(var)]
    pub character_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6008)]
pub struct ExchangeObjectModifiedInBagMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    pub object: ObjectItem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6033)]
pub struct ObjectsAddedMessage<'a> {
    pub object: std::borrow::Cow<'a, [ObjectItem<'a>]>,
}
