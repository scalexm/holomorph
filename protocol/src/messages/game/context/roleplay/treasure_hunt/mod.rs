use crate::types::game::context::roleplay::treasure_hunt::TreasureHuntFlag;
use crate::variants::TreasureHuntStepVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6509)]
pub struct TreasureHuntDigRequestAnswerFailedMessage<'a> {
    pub base: TreasureHuntDigRequestAnswerMessage<'a>,
    pub wrong_flag_count: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6491)]
pub struct TreasureHuntAvailableRetryCountUpdateMessage<'a> {
    pub quest_type: u8,
    pub available_retry_count: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6508)]
pub struct TreasureHuntFlagRequestMessage<'a> {
    pub quest_type: u8,
    pub index: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6484)]
pub struct TreasureHuntDigRequestAnswerMessage<'a> {
    pub quest_type: u8,
    pub result: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6498)]
pub struct TreasureHuntShowLegendaryUIMessage<'a> {
    #[protocol(var_contents)]
    pub available_legendary_ids: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6499)]
pub struct TreasureHuntLegendaryRequestMessage<'a> {
    #[protocol(var)]
    pub legendary_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6507)]
pub struct TreasureHuntFlagRequestAnswerMessage<'a> {
    pub quest_type: u8,
    pub result: u8,
    pub index: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6489)]
pub struct TreasureHuntRequestAnswerMessage<'a> {
    pub quest_type: u8,
    pub result: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6510)]
pub struct TreasureHuntFlagRemoveRequestMessage<'a> {
    pub quest_type: u8,
    pub index: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6483)]
pub struct TreasureHuntFinishedMessage<'a> {
    pub quest_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6492)]
pub struct PortalUseRequestMessage<'a> {
    #[protocol(var)]
    pub portal_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6487)]
pub struct TreasureHuntGiveUpRequestMessage<'a> {
    pub quest_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6486)]
pub struct TreasureHuntMessage<'a> {
    pub quest_type: u8,
    pub start_map_id: f64,
    pub known_steps_list: std::borrow::Cow<'a, [TreasureHuntStepVariant<'a>]>,
    pub total_step_count: u8,
    #[protocol(var)]
    pub check_point_current: u32,
    #[protocol(var)]
    pub check_point_total: u32,
    pub available_retry_count: i32,
    pub flags: std::borrow::Cow<'a, [TreasureHuntFlag<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6485)]
pub struct TreasureHuntDigRequestMessage<'a> {
    pub quest_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
