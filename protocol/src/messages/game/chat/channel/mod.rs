use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 890)]
pub struct ChannelEnablingMessage<'a> {
    pub channel: u8,
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 892)]
pub struct EnabledChannelsMessage<'a> {
    pub channels: &'a [u8],
    pub disallowed: &'a [u8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 891)]
pub struct ChannelEnablingChangeMessage<'a> {
    pub channel: u8,
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
