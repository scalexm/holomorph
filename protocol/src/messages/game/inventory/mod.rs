pub mod exchanges;
pub mod items;
pub mod spells;
pub mod storage;

use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6334)]
pub struct ObjectAveragePricesGetMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6335)]
pub struct ObjectAveragePricesMessage<'a> {
    #[protocol(var_contents)]
    pub ids: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub avg_prices: std::borrow::Cow<'a, [u64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6336)]
pub struct ObjectAveragePricesErrorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5537)]
pub struct KamasUpdateMessage<'a> {
    #[protocol(var)]
    pub kamas_total: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
