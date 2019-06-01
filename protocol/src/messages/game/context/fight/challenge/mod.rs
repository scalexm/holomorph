use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6019)]
pub struct ChallengeResultMessage<'a> {
    #[protocol(var)]
    pub challenge_id: u16,
    pub success: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5614)]
pub struct ChallengeTargetsListRequestMessage<'a> {
    #[protocol(var)]
    pub challenge_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6123)]
pub struct ChallengeTargetUpdateMessage<'a> {
    #[protocol(var)]
    pub challenge_id: u16,
    pub target_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5613)]
pub struct ChallengeTargetsListMessage<'a> {
    pub target_ids: std::borrow::Cow<'a, [f64]>,
    pub target_cells: std::borrow::Cow<'a, [i16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6022)]
pub struct ChallengeInfoMessage<'a> {
    #[protocol(var)]
    pub challenge_id: u16,
    pub target_id: f64,
    #[protocol(var)]
    pub xp_bonus: u32,
    #[protocol(var)]
    pub drop_bonus: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
