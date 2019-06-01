pub mod skill;
pub mod zaap;

use crate::variants::InteractiveElementSkillVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 108)]
pub struct StatedElement<'a> {
    pub element_id: u32,
    #[protocol(var)]
    pub element_cell_id: u16,
    #[protocol(var)]
    pub element_state: u32,
    pub on_current_map: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 80)]
pub struct InteractiveElement<'a> {
    pub element_id: u32,
    pub element_type_id: i32,
    pub enabled_skills: std::borrow::Cow<'a, [InteractiveElementSkillVariant<'a>]>,
    pub disabled_skills: std::borrow::Cow<'a, [InteractiveElementSkillVariant<'a>]>,
    pub on_current_map: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 398)]
pub struct InteractiveElementWithAgeBonus<'a> {
    pub base: InteractiveElement<'a>,
    pub age_bonus: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 220)]
pub struct InteractiveElementNamedSkill<'a> {
    pub base: InteractiveElementSkill<'a>,
    #[protocol(var)]
    pub name_id: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 219)]
pub struct InteractiveElementSkill<'a> {
    #[protocol(var)]
    pub skill_id: u32,
    pub skill_instance_uid: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 200)]
pub struct MapObstacle<'a> {
    #[protocol(var)]
    pub obstacle_cell_id: u16,
    pub state: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
