use crate::variants::QuestObjectiveInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 382)]
pub struct QuestActiveDetailedInformations<'a> {
    pub base: QuestActiveInformations<'a>,
    #[protocol(var)]
    pub step_id: u16,
    pub objectives: std::borrow::Cow<'a, [QuestObjectiveInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 385)]
pub struct QuestObjectiveInformations<'a> {
    #[protocol(var)]
    pub objective_id: u16,
    pub objective_status: bool,
    pub dialog_params: std::borrow::Cow<'a, [&'a str]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 384)]
pub struct GameRolePlayNpcQuestFlag<'a> {
    #[protocol(var_contents)]
    pub quests_to_valid_id: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub quests_to_start_id: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 386)]
pub struct QuestObjectiveInformationsWithCompletion<'a> {
    pub base: QuestObjectiveInformations<'a>,
    #[protocol(var)]
    pub cur_completion: u16,
    #[protocol(var)]
    pub max_completion: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 381)]
pub struct QuestActiveInformations<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
