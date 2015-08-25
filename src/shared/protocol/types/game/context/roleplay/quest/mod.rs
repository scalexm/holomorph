use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::variants::QuestObjectiveInformationsVariant;
impl_type!(GameRolePlayNpcQuestFlag, 384, quests_to_valid_id| Vec<VarShort>, quests_to_start_id| Vec<VarShort>);
impl_type!(QuestActiveDetailedInformations, 382, base| QuestActiveInformations, step_id| VarShort, objectives| Vec<QuestObjectiveInformationsVariant>);
impl_type!(QuestActiveInformations, 381, quest_id| VarShort);
impl_type!(QuestObjectiveInformations, 385, objective_id| VarShort, objective_status| bool, dialog_params| Vec<String>);
impl_type!(QuestObjectiveInformationsWithCompletion, 386, base| QuestObjectiveInformations, cur_completion| VarShort, max_completion| VarShort);
