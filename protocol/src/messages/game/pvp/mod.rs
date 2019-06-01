use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6058)]
pub struct AlignmentRankUpdateMessage<'a> {
    pub alignment_rank: u8,
    pub verbose: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6454)]
pub struct UpdateMapPlayersAgressableStatusMessage<'a> {
    #[protocol(var_contents)]
    pub player_ids: std::borrow::Cow<'a, [u64]>,
    pub enable: &'a [u8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6443)]
pub struct SetEnableAVARequestMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1810)]
pub struct SetEnablePVPRequestMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6456)]
pub struct UpdateSelfAgressableStatusMessage<'a> {
    pub status: u8,
    pub probation_time: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
