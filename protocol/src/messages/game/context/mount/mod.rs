use crate::types::game::mount::MountClientData;
use crate::types::game::paddock::PaddockItem;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5980)]
pub struct MountReleaseRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5970)]
pub struct MountXpRatioMessage<'a> {
    pub ratio: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6172)]
pub struct MountDataErrorMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6189)]
pub struct MountFeedRequestMessage<'a> {
    #[protocol(var)]
    pub mount_uid: u32,
    pub mount_location: i8,
    #[protocol(var)]
    pub mount_food_uid: u32,
    #[protocol(var)]
    pub quantity: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5987)]
pub struct MountRenameRequestMessage<'a> {
    pub name: &'a str,
    #[protocol(var)]
    pub mount_id: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5951)]
pub struct PaddockBuyRequestMessage<'a> {
    #[protocol(var)]
    pub proposed_price: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5962)]
pub struct MountSterilizeRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5990)]
pub struct GameDataPaddockObjectAddMessage<'a> {
    pub paddock_item_description: PaddockItem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5992)]
pub struct GameDataPaddockObjectListAddMessage<'a> {
    pub paddock_item_description: std::borrow::Cow<'a, [PaddockItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5982)]
pub struct MountUnSetMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6516)]
pub struct PaddockBuyResultMessage<'a> {
    pub paddock_id: f64,
    pub bought: bool,
    #[protocol(var)]
    pub real_price: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6696)]
pub struct MountHarnessDissociateRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6052)]
pub struct PaddockMoveItemRequestMessage<'a> {
    #[protocol(var)]
    pub old_cell_id: u16,
    #[protocol(var)]
    pub new_cell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5977)]
pub struct MountSterilizedMessage<'a> {
    #[protocol(var)]
    pub mount_id: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5973)]
pub struct MountDataMessage<'a> {
    pub mount_data: MountClientData<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6697)]
pub struct MountHarnessColorsUpdateRequestMessage<'a> {
    pub use_harness_colors: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5972)]
pub struct MountInformationRequestMessage<'a> {
    pub id: f64,
    pub time: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5989)]
pub struct MountSetXpRatioRequestMessage<'a> {
    pub xp_ratio: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5978)]
pub struct MountEmoteIconUsedOkMessage<'a> {
    #[protocol(var)]
    pub mount_id: i32,
    pub reaction_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6308)]
pub struct MountReleasedMessage<'a> {
    #[protocol(var)]
    pub mount_id: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5993)]
pub struct GameDataPaddockObjectRemoveMessage<'a> {
    #[protocol(var)]
    pub cell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5953)]
pub struct PaddockSellRequestMessage<'a> {
    #[protocol(var)]
    pub price: u64,
    pub for_sale: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5963)]
pub struct MountEquipedErrorMessage<'a> {
    pub error_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5975)]
pub struct MountInformationInPaddockRequestMessage<'a> {
    #[protocol(var)]
    pub map_ride_id: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5958)]
pub struct PaddockRemoveItemRequestMessage<'a> {
    #[protocol(var)]
    pub cell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5983)]
pub struct MountRenamedMessage<'a> {
    #[protocol(var)]
    pub mount_id: i32,
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5967)]
pub struct MountRidingMessage<'a> {
    #[protocol(flag)]
    pub is_riding: bool,
    #[protocol(flag)]
    pub is_autopilot: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5976)]
pub struct MountToggleRidingRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5968)]
pub struct MountSetMessage<'a> {
    pub mount_data: MountClientData<'a>,
}
