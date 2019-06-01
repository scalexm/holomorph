pub mod guild;

use crate::messages::game::context::roleplay::lockable::LockableChangeCodeMessage;
use crate::types::game::house::AccountHouseInformations;
use crate::types::game::house::HouseInformationsForSell;
use crate::variants::HouseInstanceInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5697)]
pub struct HouseSellRequestMessage<'a> {
    pub instance_id: u32,
    #[protocol(var)]
    pub amount: u64,
    pub for_sale: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5735)]
pub struct HouseBuyResultMessage<'a> {
    #[protocol(flag)]
    pub second_hand: bool,
    #[protocol(flag)]
    pub bought: bool,
    #[protocol(var)]
    pub house_id: u32,
    pub instance_id: u32,
    #[protocol(var)]
    pub real_price: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5734)]
pub struct HousePropertiesMessage<'a> {
    #[protocol(var)]
    pub house_id: u32,
    pub doors_on_map: std::borrow::Cow<'a, [u32]>,
    pub properties: HouseInstanceInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5738)]
pub struct HouseBuyRequestMessage<'a> {
    #[protocol(var)]
    pub proposed_price: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6727)]
pub struct HouseSellingUpdateMessage<'a> {
    #[protocol(var)]
    pub house_id: u32,
    pub instance_id: u32,
    pub second_hand: bool,
    #[protocol(var)]
    pub real_price: u64,
    pub buyer_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6315)]
pub struct AccountHouseMessage<'a> {
    pub houses: std::borrow::Cow<'a, [AccountHouseInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5885)]
pub struct HouseLockFromInsideRequestMessage<'a> {
    pub base: LockableChangeCodeMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5661)]
pub struct HouseKickIndoorMerchantRequestMessage<'a> {
    #[protocol(var)]
    pub cell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6140)]
pub struct HouseToSellListMessage<'a> {
    #[protocol(var)]
    pub page_index: u16,
    #[protocol(var)]
    pub total_page: u16,
    pub house_list: std::borrow::Cow<'a, [HouseInformationsForSell<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6137)]
pub struct HouseToSellFilterMessage<'a> {
    pub area_id: i32,
    pub at_least_nb_room: u8,
    pub at_least_nb_chest: u8,
    #[protocol(var)]
    pub skill_requested: u16,
    #[protocol(var)]
    pub max_price: u64,
    pub order_by: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5884)]
pub struct HouseSellFromInsideRequestMessage<'a> {
    pub base: HouseSellRequestMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5698)]
pub struct HouseKickRequestMessage<'a> {
    #[protocol(var)]
    pub id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6139)]
pub struct HouseToSellListRequestMessage<'a> {
    #[protocol(var)]
    pub page_index: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
