use crate::types::game::dare::DareCriteria;
use crate::types::game::dare::DareInformations;
use crate::types::game::dare::DareReward;
use crate::types::game::dare::DareVersatileInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6667)]
pub struct DareErrorMessage<'a> {
    pub error: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6666)]
pub struct DareSubscribeRequestMessage<'a> {
    pub dare_id: f64,
    pub subscribe: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6659)]
pub struct DareInformationsRequestMessage<'a> {
    pub dare_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6676)]
pub struct DareRewardConsumeRequestMessage<'a> {
    pub dare_id: f64,
    pub type_: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6658)]
pub struct DareSubscribedListMessage<'a> {
    pub dares_fixed_infos: std::borrow::Cow<'a, [DareInformations<'a>]>,
    pub dares_versatiles_infos: std::borrow::Cow<'a, [DareVersatileInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6681)]
pub struct DareWonMessage<'a> {
    pub dare_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6657)]
pub struct DareVersatileListMessage<'a> {
    pub dares: std::borrow::Cow<'a, [DareVersatileInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6675)]
pub struct DareRewardConsumeValidationMessage<'a> {
    pub dare_id: f64,
    pub type_: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6682)]
pub struct DareWonListMessage<'a> {
    pub dare_id: std::borrow::Cow<'a, [f64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6656)]
pub struct DareInformationsMessage<'a> {
    pub dare_fixed_infos: DareInformations<'a>,
    pub dare_versatiles_infos: DareVersatileInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6678)]
pub struct DareRewardWonMessage<'a> {
    pub reward: DareReward<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6661)]
pub struct DareListMessage<'a> {
    pub dares: std::borrow::Cow<'a, [DareInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6677)]
pub struct DareRewardsListMessage<'a> {
    pub rewards: std::borrow::Cow<'a, [DareReward<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6660)]
pub struct DareSubscribedMessage<'a> {
    #[protocol(flag)]
    pub success: bool,
    #[protocol(flag)]
    pub subscribe: bool,
    pub dare_id: f64,
    pub dare_versatiles_infos: DareVersatileInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6668)]
pub struct DareCreatedMessage<'a> {
    pub dare_infos: DareInformations<'a>,
    pub need_notifications: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6665)]
pub struct DareCreationRequestMessage<'a> {
    #[protocol(flag)]
    pub is_private: bool,
    #[protocol(flag)]
    pub is_for_guild: bool,
    #[protocol(flag)]
    pub is_for_alliance: bool,
    #[protocol(flag)]
    pub need_notifications: bool,
    #[protocol(var)]
    pub subscription_fee: u64,
    #[protocol(var)]
    pub jackpot: u64,
    pub max_count_winners: u16,
    pub delay_before_start: u32,
    pub duration: u32,
    pub criterions: std::borrow::Cow<'a, [DareCriteria<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6679)]
pub struct DareCanceledMessage<'a> {
    pub dare_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6680)]
pub struct DareCancelRequestMessage<'a> {
    pub dare_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6663)]
pub struct DareCreatedListMessage<'a> {
    pub dares_fixed_infos: std::borrow::Cow<'a, [DareInformations<'a>]>,
    pub dares_versatiles_infos: std::borrow::Cow<'a, [DareVersatileInformations<'a>]>,
}
