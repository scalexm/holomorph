use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 102)]
pub struct SkillActionDescription<'a> {
    #[protocol(var)]
    pub skill_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 103)]
pub struct SkillActionDescriptionTimed<'a> {
    pub base: SkillActionDescription<'a>,
    pub time: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 99)]
pub struct SkillActionDescriptionCollect<'a> {
    pub base: SkillActionDescriptionTimed<'a>,
    #[protocol(var)]
    pub min: u16,
    #[protocol(var)]
    pub max: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 100)]
pub struct SkillActionDescriptionCraft<'a> {
    pub base: SkillActionDescription<'a>,
    pub probability: u8,
}
