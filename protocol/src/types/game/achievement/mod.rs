use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 404)]
pub struct AchievementObjective<'a> {
    #[protocol(var)]
    pub id: u32,
    #[protocol(var)]
    pub max_value: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 514)]
pub struct AchievementAchieved<'a> {
    #[protocol(var)]
    pub id: u16,
    #[protocol(var)]
    pub achieved_by: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 515)]
pub struct AchievementAchievedRewardable<'a> {
    pub base: AchievementAchieved<'a>,
    #[protocol(var)]
    pub finishedlevel: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 402)]
pub struct AchievementStartedObjective<'a> {
    pub base: AchievementObjective<'a>,
    #[protocol(var)]
    pub value: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 363)]
pub struct Achievement<'a> {
    #[protocol(var)]
    pub id: u16,
    pub finished_objective: std::borrow::Cow<'a, [AchievementObjective<'a>]>,
    pub started_objectives: std::borrow::Cow<'a, [AchievementStartedObjective<'a>]>,
}
