use crate::types::game::context::roleplay::job::JobBookSubscription;
use crate::types::game::context::roleplay::job::JobCrafterDirectoryEntryJobInfo;
use crate::types::game::context::roleplay::job::JobCrafterDirectoryEntryPlayerInfo;
use crate::types::game::context::roleplay::job::JobCrafterDirectoryListEntry;
use crate::types::game::context::roleplay::job::JobCrafterDirectorySettings;
use crate::types::game::context::roleplay::job::JobDescription;
use crate::types::game::context::roleplay::job::JobExperience;
use crate::types::game::look::EntityLook;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5652)]
pub struct JobCrafterDirectorySettingsMessage<'a> {
    pub crafters_settings: std::borrow::Cow<'a, [JobCrafterDirectorySettings<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5656)]
pub struct JobLevelUpMessage<'a> {
    pub new_level: u8,
    pub jobs_description: JobDescription<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6044)]
pub struct JobCrafterDirectoryEntryMessage<'a> {
    pub player_info: JobCrafterDirectoryEntryPlayerInfo<'a>,
    pub job_info_list: std::borrow::Cow<'a, [JobCrafterDirectoryEntryJobInfo<'a>]>,
    pub player_look: EntityLook<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5748)]
pub struct JobAllowMultiCraftRequestMessage<'a> {
    pub enabled: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6046)]
pub struct JobCrafterDirectoryListMessage<'a> {
    pub list_entries: std::borrow::Cow<'a, [JobCrafterDirectoryListEntry<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5651)]
pub struct JobCrafterDirectoryAddMessage<'a> {
    pub list_entry: JobCrafterDirectoryListEntry<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6047)]
pub struct JobCrafterDirectoryListRequestMessage<'a> {
    pub job_id: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5653)]
pub struct JobCrafterDirectoryRemoveMessage<'a> {
    pub job_id: u8,
    #[protocol(var)]
    pub player_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5747)]
pub struct JobMultiCraftAvailableSkillsMessage<'a> {
    pub base: JobAllowMultiCraftRequestMessage<'a>,
    #[protocol(var)]
    pub player_id: u64,
    #[protocol(var_contents)]
    pub skills: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5649)]
pub struct JobCrafterDirectoryDefineSettingsMessage<'a> {
    pub settings: JobCrafterDirectorySettings<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6599)]
pub struct JobExperienceOtherPlayerUpdateMessage<'a> {
    pub base: JobExperienceUpdateMessage<'a>,
    #[protocol(var)]
    pub player_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6593)]
pub struct JobBookSubscriptionMessage<'a> {
    pub subscriptions: std::borrow::Cow<'a, [JobBookSubscription<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5809)]
pub struct JobExperienceMultiUpdateMessage<'a> {
    pub experiences_update: std::borrow::Cow<'a, [JobExperience<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5655)]
pub struct JobDescriptionMessage<'a> {
    pub jobs_description: std::borrow::Cow<'a, [JobDescription<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5654)]
pub struct JobExperienceUpdateMessage<'a> {
    pub experiences_update: JobExperience<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6043)]
pub struct JobCrafterDirectoryEntryRequestMessage<'a> {
    #[protocol(var)]
    pub player_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
