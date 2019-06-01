use crate::types::game::context::roleplay::quest::QuestActiveDetailedInformations;
use crate::variants::QuestActiveInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6717)]
pub struct FollowedQuestsMessage<'a> {
    pub quests: std::borrow::Cow<'a, [QuestActiveDetailedInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6724)]
pub struct FollowQuestObjectiveRequestMessage<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    pub objective_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5625)]
pub struct QuestStepInfoMessage<'a> {
    pub infos: QuestActiveInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5626)]
pub struct QuestListMessage<'a> {
    #[protocol(var_contents)]
    pub finished_quests_ids: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub finished_quests_counts: std::borrow::Cow<'a, [u16]>,
    pub active_quests: std::borrow::Cow<'a, [QuestActiveInformationsVariant<'a>]>,
    #[protocol(var_contents)]
    pub reinit_done_quests_ids: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6097)]
pub struct QuestValidatedMessage<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5643)]
pub struct QuestStartRequestMessage<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6085)]
pub struct QuestObjectiveValidationMessage<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    #[protocol(var)]
    pub objective_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6723)]
pub struct UnfollowQuestObjectiveRequestMessage<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    pub objective_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5622)]
pub struct QuestStepInfoRequestMessage<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6722)]
pub struct RefreshFollowedQuestsOrderRequestMessage<'a> {
    #[protocol(var_contents)]
    pub quests: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5623)]
pub struct QuestListRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6099)]
pub struct QuestStepValidatedMessage<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    #[protocol(var)]
    pub step_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6088)]
pub struct GuidedModeReturnRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6096)]
pub struct QuestStepStartedMessage<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    #[protocol(var)]
    pub step_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6092)]
pub struct GuidedModeQuitRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6098)]
pub struct QuestObjectiveValidatedMessage<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    #[protocol(var)]
    pub objective_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6091)]
pub struct QuestStartedMessage<'a> {
    #[protocol(var)]
    pub quest_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
