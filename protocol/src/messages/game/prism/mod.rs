use crate::types::game::prism::PrismFightersInformation;
use crate::variants::CharacterMinimalPlusLookInformationsVariant;
use crate::variants::PrismSubareaEmptyInfoVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5893)]
pub struct PrismFightAttackerAddMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    #[protocol(var)]
    pub fight_id: u16,
    pub attacker: CharacterMinimalPlusLookInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6531)]
pub struct PrismModuleExchangeRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5853)]
pub struct PrismInfoCloseMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6453)]
pub struct PrismFightRemovedMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6437)]
pub struct PrismSettingsRequestMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    pub start_defense_time: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6041)]
pub struct PrismUseRequestMessage<'a> {
    pub module_to_use: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6040)]
pub struct PrismFightStateUpdateMessage<'a> {
    pub state: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5892)]
pub struct PrismFightDefenderLeaveMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    #[protocol(var)]
    pub fight_id: u16,
    #[protocol(var)]
    pub fighter_to_remove_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5859)]
pub struct PrismInfoInValidMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6438)]
pub struct PrismsListUpdateMessage<'a> {
    pub base: PrismsListMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5901)]
pub struct PrismFightSwapRequestMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    #[protocol(var)]
    pub target_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5895)]
pub struct PrismFightDefenderAddMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    #[protocol(var)]
    pub fight_id: u16,
    pub defender: CharacterMinimalPlusLookInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6451)]
pub struct PrismsInfoValidMessage<'a> {
    pub fights: std::borrow::Cow<'a, [PrismFightersInformation<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5897)]
pub struct PrismFightAttackerRemoveMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    #[protocol(var)]
    pub fight_id: u16,
    #[protocol(var)]
    pub fighter_to_remove_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6468)]
pub struct PrismSetSabotagedRequestMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6452)]
pub struct PrismFightAddedMessage<'a> {
    pub fight: PrismFightersInformation<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6466)]
pub struct PrismSetSabotagedRefusedMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    pub reason: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6440)]
pub struct PrismsListMessage<'a> {
    pub prisms: std::borrow::Cow<'a, [PrismSubareaEmptyInfoVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5843)]
pub struct PrismFightJoinLeaveRequestMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    pub join: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5844)]
pub struct PrismInfoJoinLeaveRequestMessage<'a> {
    pub join: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6441)]
pub struct PrismsListRegisterMessage<'a> {
    pub listen: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6442)]
pub struct PrismSettingsErrorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6042)]
pub struct PrismAttackRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
