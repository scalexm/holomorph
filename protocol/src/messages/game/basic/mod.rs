use crate::variants::AbstractSocialGroupInfosVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 780)]
pub struct TextInformationMessage<'a> {
    pub msg_type: u8,
    #[protocol(var)]
    pub msg_id: u16,
    pub parameters: std::borrow::Cow<'a, [&'a str]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6298)]
pub struct NumericWhoIsRequestMessage<'a> {
    #[protocol(var)]
    pub player_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 180)]
pub struct BasicWhoIsMessage<'a> {
    #[protocol(flag)]
    pub self_: bool,
    #[protocol(flag)]
    pub verbose: bool,
    pub position: i8,
    pub account_nickname: &'a str,
    pub account_id: u32,
    pub player_name: &'a str,
    #[protocol(var)]
    pub player_id: u64,
    pub area_id: i16,
    pub server_id: i16,
    pub origin_server_id: i16,
    pub social_groups: std::borrow::Cow<'a, [AbstractSocialGroupInfosVariant<'a>]>,
    pub player_state: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6362)]
pub struct BasicAckMessage<'a> {
    #[protocol(var)]
    pub seq: u32,
    #[protocol(var)]
    pub last_packet_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 181)]
pub struct BasicWhoIsRequestMessage<'a> {
    pub verbose: bool,
    pub search: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 176)]
pub struct BasicNoOperationMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6525)]
pub struct CurrentServerStatusUpdateMessage<'a> {
    pub status: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6316)]
pub struct SequenceNumberRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6297)]
pub struct NumericWhoIsMessage<'a> {
    #[protocol(var)]
    pub player_id: u64,
    pub account_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5816)]
pub struct BasicLatencyStatsRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5663)]
pub struct BasicLatencyStatsMessage<'a> {
    pub latency: u16,
    #[protocol(var)]
    pub sample_count: u16,
    #[protocol(var)]
    pub max: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6317)]
pub struct SequenceNumberMessage<'a> {
    pub number: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 177)]
pub struct BasicDateMessage<'a> {
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 179)]
pub struct BasicWhoIsNoMatchMessage<'a> {
    pub search: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5664)]
pub struct BasicWhoAmIRequestMessage<'a> {
    pub verbose: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 175)]
pub struct BasicTimeMessage<'a> {
    pub timestamp: f64,
    pub timezone_offset: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
