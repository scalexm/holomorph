use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 484)]
pub struct StatisticData<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 485)]
pub struct StatisticDataInt<'a> {
    pub base: StatisticData<'a>,
    pub value: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 487)]
pub struct StatisticDataString<'a> {
    pub base: StatisticData<'a>,
    pub value: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 482)]
pub struct StatisticDataBoolean<'a> {
    pub base: StatisticData<'a>,
    pub value: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 486)]
pub struct StatisticDataByte<'a> {
    pub base: StatisticData<'a>,
    pub value: i8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 488)]
pub struct StatisticDataShort<'a> {
    pub base: StatisticData<'a>,
    pub value: i16,
}
