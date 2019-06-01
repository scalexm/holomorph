use crate::types::game::paddock::PaddockInformationsForSell;
use crate::types::game::paddock::PaddockInstancesInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5824)]
pub struct PaddockPropertiesMessage<'a> {
    pub properties: PaddockInstancesInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6141)]
pub struct PaddockToSellListRequestMessage<'a> {
    #[protocol(var)]
    pub page_index: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6026)]
pub struct GameDataPlayFarmObjectAnimationMessage<'a> {
    #[protocol(var_contents)]
    pub cell_id: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6161)]
pub struct PaddockToSellFilterMessage<'a> {
    pub area_id: i32,
    pub at_least_nb_mount: i8,
    pub at_least_nb_machine: i8,
    #[protocol(var)]
    pub max_price: u64,
    pub order_by: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6018)]
pub struct PaddockSellBuyDialogMessage<'a> {
    pub bsell: bool,
    #[protocol(var)]
    pub owner_id: u32,
    #[protocol(var)]
    pub price: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6138)]
pub struct PaddockToSellListMessage<'a> {
    #[protocol(var)]
    pub page_index: u16,
    #[protocol(var)]
    pub total_page: u16,
    pub paddock_list: std::borrow::Cow<'a, [PaddockInformationsForSell<'a>]>,
}
