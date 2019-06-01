use crate::variants::StatisticDataVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6573)]
pub struct BasicStatWithDataMessage<'a> {
    pub base: BasicStatMessage<'a>,
    pub datas: std::borrow::Cow<'a, [StatisticDataVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 183)]
pub struct BasicPongMessage<'a> {
    pub quiet: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6530)]
pub struct BasicStatMessage<'a> {
    pub time_spent: f64,
    #[protocol(var)]
    pub stat_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 182)]
pub struct BasicPingMessage<'a> {
    pub quiet: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
