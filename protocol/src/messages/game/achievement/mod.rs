use crate::types::game::achievement::Achievement;
use crate::types::game::achievement::AchievementAchievedRewardable;
use crate::variants::AchievementAchievedVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6376)]
pub struct AchievementRewardSuccessMessage<'a> {
    pub achievement_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6381)]
pub struct AchievementFinishedInformationMessage<'a> {
    pub base: AchievementFinishedMessage<'a>,
    pub name: &'a str,
    #[protocol(var)]
    pub player_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6205)]
pub struct AchievementListMessage<'a> {
    pub finished_achievements: std::borrow::Cow<'a, [AchievementAchievedVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6358)]
pub struct AchievementDetailedListMessage<'a> {
    pub started_achievements: std::borrow::Cow<'a, [Achievement<'a>]>,
    pub finished_achievements: std::borrow::Cow<'a, [Achievement<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6375)]
pub struct AchievementRewardErrorMessage<'a> {
    pub achievement_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6377)]
pub struct AchievementRewardRequestMessage<'a> {
    pub achievement_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6378)]
pub struct AchievementDetailsMessage<'a> {
    pub achievement: Achievement<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6357)]
pub struct AchievementDetailedListRequestMessage<'a> {
    #[protocol(var)]
    pub category_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6383)]
pub struct FriendGuildWarnOnAchievementCompleteStateMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6208)]
pub struct AchievementFinishedMessage<'a> {
    pub achievement: AchievementAchievedRewardable<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6380)]
pub struct AchievementDetailsRequestMessage<'a> {
    #[protocol(var)]
    pub achievement_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6382)]
pub struct FriendGuildSetWarnOnAchievementCompleteMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
