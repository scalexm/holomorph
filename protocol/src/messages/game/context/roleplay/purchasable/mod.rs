use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5739)]
pub struct PurchasableDialogMessage<'a> {
    #[protocol(flag)]
    pub buy_or_sell: bool,
    #[protocol(flag)]
    pub second_hand: bool,
    pub purchasable_id: f64,
    pub purchasable_instance_id: u32,
    #[protocol(var)]
    pub price: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
