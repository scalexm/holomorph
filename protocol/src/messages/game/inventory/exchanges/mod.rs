use crate::messages::game::dialog::LeaveDialogMessage;
use crate::types::game::context::roleplay::job::DecraftedItemStackInfo;
use crate::types::game::data::items::BidExchangerObjectInfo;
use crate::types::game::data::items::ObjectItem;
use crate::types::game::data::items::ObjectItemGenericQuantity;
use crate::types::game::data::items::ObjectItemGenericQuantityPrice;
use crate::types::game::data::items::ObjectItemNotInContainer;
use crate::types::game::data::items::ObjectItemToSell;
use crate::types::game::data::items::ObjectItemToSellInBid;
use crate::types::game::data::items::ObjectItemToSellInHumanVendorShop;
use crate::types::game::data::items::ObjectItemToSellInNpcShop;
use crate::types::game::data::items::SellerBuyerDescriptor;
use crate::types::game::inventory::exchanges::RecycledItem;
use crate::types::game::mount::MountClientData;
use crate::variants::ObjectEffectVariant;
use crate::variants::UpdateMountCharacteristicVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5772)]
pub struct ExchangeOnHumanVendorRequestMessage<'a> {
    #[protocol(var)]
    pub human_vendor_id: u64,
    #[protocol(var)]
    pub human_vendor_cell: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5505)]
pub struct ExchangeRequestMessage<'a> {
    pub exchange_type: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5946)]
pub struct ExchangeBidHouseItemRemoveOkMessage<'a> {
    pub seller_id: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5941)]
pub struct ExchangeStartOkCraftWithInformationMessage<'a> {
    pub base: ExchangeStartOkCraftMessage<'a>,
    #[protocol(var)]
    pub skill_id: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6779)]
pub struct EvolutiveObjectRecycleResultMessage<'a> {
    pub recycled_items: std::borrow::Cow<'a, [RecycledItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6753)]
pub struct UpdateMountCharacteristicsMessage<'a> {
    #[protocol(var)]
    pub ride_id: i32,
    pub boost_to_update_list: std::borrow::Cow<'a, [UpdateMountCharacteristicVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5775)]
pub struct ExchangeStartAsVendorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5986)]
pub struct ExchangeRequestOnMountStockMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5513)]
pub struct ExchangeErrorMessage<'a> {
    pub error_type: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6032)]
pub struct ExchangeObjectTransfertAllToInvMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5753)]
pub struct ExchangeRequestOnShopStockMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5904)]
pub struct ExchangeStartedBidBuyerMessage<'a> {
    pub buyer_descriptor: SellerBuyerDescriptor<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6589)]
pub struct ExchangeStoppedMessage<'a> {
    #[protocol(var)]
    pub id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6038)]
pub struct ExchangeShopStockMultiMovementUpdatedMessage<'a> {
    pub object_info_list: std::borrow::Cow<'a, [ObjectItemToSell<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5516)]
pub struct ExchangeObjectAddedMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    pub object: ObjectItem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5818)]
pub struct ExchangeStartOkMulticraftCrafterMessage<'a> {
    #[protocol(var)]
    pub skill_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6834)]
pub struct ExchangeMoneyMovementInformationMessage<'a> {
    #[protocol(var)]
    pub limit: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6004)]
pub struct ExchangeObjectUseInWorkshopMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    #[protocol(var)]
    pub quantity: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6595)]
pub struct ExchangeCraftCountModifiedMessage<'a> {
    #[protocol(var)]
    pub count: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5810)]
pub struct ExchangeItemAutoCraftStopedMessage<'a> {
    pub reason: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5512)]
pub struct ExchangeStartedMessage<'a> {
    pub exchange_type: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6578)]
pub struct ExchangeCraftPaymentModifiedMessage<'a> {
    #[protocol(var)]
    pub gold_sum: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6612)]
pub struct ExchangeBidHouseUnsoldItemsMessage<'a> {
    pub items: std::borrow::Cow<'a, [ObjectItemGenericQuantity<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5520)]
pub struct ExchangeObjectMoveKamaMessage<'a> {
    #[protocol(var)]
    pub quantity: i64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5765)]
pub struct ExchangeTypesExchangerDescriptionForUserMessage<'a> {
    #[protocol(var_contents)]
    pub type_description: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5773)]
pub struct ExchangePlayerRequestMessage<'a> {
    pub base: ExchangeRequestMessage<'a>,
    #[protocol(var)]
    pub target: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6337)]
pub struct ExchangeBidHouseInListUpdatedMessage<'a> {
    pub base: ExchangeBidHouseInListAddedMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5628)]
pub struct ExchangeLeaveMessage<'a> {
    pub base: LeaveDialogMessage<'a>,
    pub success: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6056)]
pub struct ExchangeMountSterilizeFromPaddockMessage<'a> {
    pub name: &'a str,
    pub world_x: i16,
    pub world_y: i16,
    pub sterilizator: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6569)]
pub struct DecraftResultMessage<'a> {
    pub results: std::borrow::Cow<'a, [DecraftedItemStackInfo<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6559)]
pub struct ExchangeMountsPaddockRemoveMessage<'a> {
    #[protocol(var_contents)]
    pub mounts_id: std::borrow::Cow<'a, [i32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6325)]
pub struct ExchangeObjectTransfertExistingFromInvMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6613)]
pub struct ExchangeOfflineSoldItemsMessage<'a> {
    pub bid_house_items: std::borrow::Cow<'a, [ObjectItemGenericQuantityPrice<'a>]>,
    pub merchant_items: std::borrow::Cow<'a, [ObjectItemGenericQuantityPrice<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6236)]
pub struct ExchangeStartedWithStorageMessage<'a> {
    pub base: ExchangeStartedMessage<'a>,
    #[protocol(var)]
    pub storage_max_slot: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6567)]
pub struct ExchangeStartOkRunesTradeMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5768)]
pub struct ExchangeOkMultiCraftMessage<'a> {
    #[protocol(var)]
    pub initiator_id: u64,
    #[protocol(var)]
    pub other_id: u64,
    pub role: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5790)]
pub struct ExchangeCraftResultMessage<'a> {
    pub craft_result: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5509)]
pub struct ExchangeIsReadyMessage<'a> {
    pub id: f64,
    pub ready: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6326)]
pub struct ExchangeObjectTransfertExistingToInvMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6183)]
pub struct ExchangeObjectTransfertListFromInvMessage<'a> {
    #[protocol(var_contents)]
    pub ids: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6601)]
pub struct RecycleResultMessage<'a> {
    #[protocol(var)]
    pub nuggets_for_prism: u32,
    #[protocol(var)]
    pub nuggets_for_player: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6592)]
pub struct JobBookSubscribeRequestMessage<'a> {
    pub job_ids: &'a [u8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6600)]
pub struct ExchangeStartOkRecycleTradeMessage<'a> {
    pub percent_to_prism: u16,
    pub percent_to_player: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5805)]
pub struct ExchangeBidHousePriceMessage<'a> {
    #[protocol(var)]
    pub gen_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6701)]
pub struct FocusedExchangeReadyMessage<'a> {
    pub base: ExchangeReadyMessage<'a>,
    #[protocol(var)]
    pub focus_action_id: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6555)]
pub struct ExchangeMountsStableAddMessage<'a> {
    pub mount_description: std::borrow::Cow<'a, [MountClientData<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5783)]
pub struct ExchangeShowVendorTaxMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6055)]
pub struct ExchangeMountFreeFromPaddockMessage<'a> {
    pub name: &'a str,
    pub world_x: i16,
    pub world_y: i16,
    pub liberator: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5752)]
pub struct ExchangeTypesItemsExchangerDescriptionForUserMessage<'a> {
    pub item_type_descriptions: std::borrow::Cow<'a, [BidExchangerObjectInfo<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5991)]
pub struct ExchangeStartOkMountWithOutPaddockMessage<'a> {
    pub stabled_mounts_description: std::borrow::Cow<'a, [MountClientData<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6554)]
pub struct ExchangeMountsTakenFromPaddockMessage<'a> {
    pub name: &'a str,
    pub world_x: i16,
    pub world_y: i16,
    pub ownername: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6238)]
pub struct ExchangeObjectModifyPricedMessage<'a> {
    pub base: ExchangeObjectMovePricedMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5981)]
pub struct ExchangeMountStableErrorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5819)]
pub struct ExchangeStartOkJobIndexMessage<'a> {
    #[protocol(var_contents)]
    pub jobs: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5909)]
pub struct ExchangeShopStockMovementUpdatedMessage<'a> {
    pub object_info: ObjectItemToSell<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5817)]
pub struct ExchangeStartOkMulticraftCustomerMessage<'a> {
    #[protocol(var)]
    pub skill_id: u32,
    pub crafter_job_level: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5945)]
pub struct ExchangeBidHouseItemAddOkMessage<'a> {
    pub item_info: ObjectItemToSellInBid<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5999)]
pub struct ExchangeCraftResultWithObjectDescMessage<'a> {
    pub base: ExchangeCraftResultMessage<'a>,
    pub object_info: ObjectItemNotInContainer<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6598)]
pub struct ExchangeCrafterJobLevelupMessage<'a> {
    pub crafter_job_level: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5769)]
pub struct ItemNoMoreAvailableMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6557)]
pub struct ExchangeMountsStableBornAddMessage<'a> {
    pub base: ExchangeMountsStableAddMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5793)]
pub struct ExchangeWeightMessage<'a> {
    #[protocol(var)]
    pub current_weight: u32,
    #[protocol(var)]
    pub max_weight: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5910)]
pub struct ExchangeShopStockStartedMessage<'a> {
    pub objects_infos: std::borrow::Cow<'a, [ObjectItemToSell<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5979)]
pub struct ExchangeStartOkMountMessage<'a> {
    pub base: ExchangeStartOkMountWithOutPaddockMessage<'a>,
    pub paddocked_mounts_description: std::borrow::Cow<'a, [MountClientData<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5518)]
pub struct ExchangeObjectMoveMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    #[protocol(var)]
    pub quantity: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6272)]
pub struct ExchangeBidHouseBuyResultMessage<'a> {
    #[protocol(var)]
    pub uid: u32,
    pub bought: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5786)]
pub struct ExchangeWaitingResultMessage<'a> {
    pub bwait: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5759)]
pub struct ExchangeBuyOkMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5802)]
pub struct ExchangeBidSearchOkMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6556)]
pub struct ExchangeMountsStableRemoveMessage<'a> {
    #[protocol(var_contents)]
    pub mounts_id: std::borrow::Cow<'a, [i32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5761)]
pub struct ExchangeStartOkNpcShopMessage<'a> {
    pub npc_seller_id: f64,
    #[protocol(var)]
    pub token_id: u16,
    pub objects_infos: std::borrow::Cow<'a, [ObjectItemToSellInNpcShop<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6001)]
pub struct ExchangeReplayStopMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5762)]
pub struct ExchangeGuildTaxCollectorGetMessage<'a> {
    pub collector_name: &'a str,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub user_name: &'a str,
    #[protocol(var)]
    pub caller_id: u64,
    pub caller_name: &'a str,
    pub experience: f64,
    #[protocol(var)]
    pub pods: u16,
    pub objects_infos: std::borrow::Cow<'a, [ObjectItemGenericQuantity<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6464)]
pub struct ExchangeBidPriceForSellerMessage<'a> {
    pub base: ExchangeBidPriceMessage<'a>,
    pub all_identical: bool,
    #[protocol(var_contents)]
    pub minimal_prices: std::borrow::Cow<'a, [u64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6535)]
pub struct ExchangeObjectsAddedMessage<'a> {
    pub base: ExchangeObjectMessage<'a>,
    pub object: std::borrow::Cow<'a, [ObjectItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6039)]
pub struct ExchangeObjectTransfertListToInvMessage<'a> {
    #[protocol(var_contents)]
    pub ids: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5785)]
pub struct ExchangeStartOkNpcTradeMessage<'a> {
    pub npc_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6597)]
pub struct ExchangeCraftCountRequestMessage<'a> {
    #[protocol(var)]
    pub count: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6184)]
pub struct ExchangeObjectTransfertAllFromInvMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5984)]
pub struct ExchangeStartedMountStockMessage<'a> {
    pub objects_infos: std::borrow::Cow<'a, [ObjectItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6000)]
pub struct ExchangeCraftResultWithObjectIdMessage<'a> {
    pub base: ExchangeCraftResultMessage<'a>,
    #[protocol(var)]
    pub object_generic_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6129)]
pub struct ExchangeStartedWithPodsMessage<'a> {
    pub base: ExchangeStartedMessage<'a>,
    pub first_character_id: f64,
    #[protocol(var)]
    pub first_character_current_weight: u32,
    #[protocol(var)]
    pub first_character_max_weight: u32,
    pub second_character_id: f64,
    #[protocol(var)]
    pub second_character_current_weight: u32,
    #[protocol(var)]
    pub second_character_max_weight: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5948)]
pub struct ExchangeBidHouseGenericItemRemovedMessage<'a> {
    #[protocol(var)]
    pub obj_generic_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6389)]
pub struct ExchangeSetCraftRecipeMessage<'a> {
    #[protocol(var)]
    pub object_gid: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5514)]
pub struct ExchangeObjectMovePricedMessage<'a> {
    pub base: ExchangeObjectMoveMessage<'a>,
    #[protocol(var)]
    pub price: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5508)]
pub struct ExchangeAcceptMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6470)]
pub struct ExchangeObjectTransfertListWithQuantityToInvMessage<'a> {
    #[protocol(var_contents)]
    pub ids: std::borrow::Cow<'a, [u32]>,
    #[protocol(var_contents)]
    pub qtys: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6778)]
pub struct ExchangeStartOkEvolutiveObjectRecycleTradeMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5907)]
pub struct ExchangeShopStockMovementRemovedMessage<'a> {
    #[protocol(var)]
    pub object_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5949)]
pub struct ExchangeBidHouseInListAddedMessage<'a> {
    pub item_uid: i32,
    pub obj_generic_id: i32,
    pub effects: std::borrow::Cow<'a, [ObjectEffectVariant<'a>]>,
    #[protocol(var_contents)]
    pub prices: std::borrow::Cow<'a, [u64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5511)]
pub struct ExchangeReadyMessage<'a> {
    pub ready: bool,
    #[protocol(var)]
    pub step: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5806)]
pub struct ExchangeBidHouseSearchMessage<'a> {
    #[protocol(var)]
    pub type_: u32,
    #[protocol(var)]
    pub gen_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5803)]
pub struct ExchangeBidHouseTypeMessage<'a> {
    #[protocol(var)]
    pub type_: u32,
    pub follow: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5947)]
pub struct ExchangeBidHouseGenericItemAddedMessage<'a> {
    #[protocol(var)]
    pub obj_generic_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5807)]
pub struct ExchangeBidHouseListMessage<'a> {
    #[protocol(var)]
    pub id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5767)]
pub struct ExchangeStartOkHumanVendorMessage<'a> {
    pub seller_id: f64,
    pub objects_infos: std::borrow::Cow<'a, [ObjectItemToSellInHumanVendorShop<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5522)]
pub struct ExchangeRequestedMessage<'a> {
    pub exchange_type: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6188)]
pub struct ExchangeCraftResultMagicWithObjectDescMessage<'a> {
    pub base: ExchangeCraftResultWithObjectDescMessage<'a>,
    pub magic_pool_status: i8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5523)]
pub struct ExchangeRequestedTradeMessage<'a> {
    pub base: ExchangeRequestedMessage<'a>,
    #[protocol(var)]
    pub source: u64,
    #[protocol(var)]
    pub target: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5784)]
pub struct ExchangePlayerMultiCraftRequestMessage<'a> {
    pub base: ExchangeRequestMessage<'a>,
    #[protocol(var)]
    pub target: u64,
    #[protocol(var)]
    pub skill_id: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5515)]
pub struct ExchangeObjectMessage<'a> {
    pub remote: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5787)]
pub struct ExchangeReplyTaxVendorMessage<'a> {
    #[protocol(var)]
    pub object_value: u64,
    #[protocol(var)]
    pub total_tax_value: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6561)]
pub struct ExchangeMountsPaddockAddMessage<'a> {
    pub mount_description: std::borrow::Cow<'a, [MountClientData<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6037)]
pub struct ExchangeShopStockMultiMovementRemovedMessage<'a> {
    #[protocol(var_contents)]
    pub object_id_list: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5774)]
pub struct ExchangeBuyMessage<'a> {
    #[protocol(var)]
    pub object_to_buy_id: u32,
    #[protocol(var)]
    pub quantity: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5779)]
pub struct ExchangeRequestOnTaxCollectorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5905)]
pub struct ExchangeStartedBidSellerMessage<'a> {
    pub seller_descriptor: SellerBuyerDescriptor<'a>,
    pub objects_infos: std::borrow::Cow<'a, [ObjectItemToSellInBid<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6579)]
pub struct ExchangeCraftPaymentModificationRequestMessage<'a> {
    #[protocol(var)]
    pub quantity: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5792)]
pub struct ExchangeSellOkMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5950)]
pub struct ExchangeBidHouseInListRemovedMessage<'a> {
    pub item_uid: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5804)]
pub struct ExchangeBidHouseBuyMessage<'a> {
    #[protocol(var)]
    pub uid: u32,
    #[protocol(var)]
    pub qty: u32,
    #[protocol(var)]
    pub price: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6664)]
pub struct ExchangeStartedTaxCollectorShopMessage<'a> {
    pub objects: std::borrow::Cow<'a, [ObjectItem<'a>]>,
    #[protocol(var)]
    pub kamas: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5778)]
pub struct ExchangeSellMessage<'a> {
    #[protocol(var)]
    pub object_to_sell_id: u32,
    #[protocol(var)]
    pub quantity: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6752)]
pub struct ExchangeHandleMountsMessage<'a> {
    pub action_type: i8,
    #[protocol(var_contents)]
    pub rides_id: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5813)]
pub struct ExchangeStartOkCraftMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5794)]
pub struct ExchangeCraftInformationObjectMessage<'a> {
    pub base: ExchangeCraftResultWithObjectIdMessage<'a>,
    #[protocol(var)]
    pub player_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5755)]
pub struct ExchangeBidPriceMessage<'a> {
    #[protocol(var)]
    pub generic_id: u16,
    #[protocol(var)]
    pub average_price: i64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
