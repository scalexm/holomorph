use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5609)]
pub struct StatsUpgradeResultMessage<'a> {
    pub result: i8,
    #[protocol(var)]
    pub nb_charac_boost: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5610)]
pub struct StatsUpgradeRequestMessage<'a> {
    pub use_additionnal: bool,
    pub stat_id: u8,
    #[protocol(var)]
    pub boost_point: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
