use crate::variants::PlayerStatusVariant;
use crate::variants::SkillActionDescriptionVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 195)]
pub struct JobCrafterDirectoryEntryJobInfo<'a> {
    pub job_id: u8,
    pub job_level: u8,
    pub free: bool,
    pub min_level: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 97)]
pub struct JobCrafterDirectorySettings<'a> {
    pub job_id: u8,
    pub min_level: u8,
    pub free: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 500)]
pub struct JobBookSubscription<'a> {
    pub job_id: u8,
    pub subscribed: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 194)]
pub struct JobCrafterDirectoryEntryPlayerInfo<'a> {
    #[protocol(var)]
    pub player_id: u64,
    pub player_name: &'a str,
    pub alignment_side: i8,
    pub breed: i8,
    pub sex: bool,
    pub is_in_workshop: bool,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub status: PlayerStatusVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 101)]
pub struct JobDescription<'a> {
    pub job_id: u8,
    pub skills: std::borrow::Cow<'a, [SkillActionDescriptionVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 196)]
pub struct JobCrafterDirectoryListEntry<'a> {
    pub player_info: JobCrafterDirectoryEntryPlayerInfo<'a>,
    pub job_info: JobCrafterDirectoryEntryJobInfo<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 98)]
pub struct JobExperience<'a> {
    pub job_id: u8,
    pub job_level: u8,
    #[protocol(var)]
    pub job_x_p: u64,
    #[protocol(var)]
    pub job_xp_level_floor: u64,
    #[protocol(var)]
    pub job_xp_next_level_floor: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 481)]
pub struct DecraftedItemStackInfo<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    pub bonus_min: f32,
    pub bonus_max: f32,
    #[protocol(var_contents)]
    pub runes_id: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub runes_qty: std::borrow::Cow<'a, [u32]>,
}
