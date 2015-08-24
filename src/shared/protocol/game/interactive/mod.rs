use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(InteractiveElementUpdatedMessage, 5708, interactive_element| InteractiveElement);
impl_type!(InteractiveMapUpdateMessage, 5002, interactive_elements| Vec<InteractiveElementVariant>);
impl_type!(InteractiveUsedMessage, 5745, entity_id| VarInt, elem_id| VarInt, skill_id| VarShort, duration| VarShort);
impl_type!(InteractiveUseEndedMessage, 6112, elem_id| VarInt, skill_id| VarShort);
impl_type!(InteractiveUseErrorMessage, 6384, elem_id| VarInt, skill_instance_uid| VarInt);
impl_type!(InteractiveUseRequestMessage, 5001, elem_id| VarInt, skill_instance_uid| VarInt);
impl_type!(StatedElementUpdatedMessage, 5709, stated_element| StatedElement);
impl_type!(StatedMapUpdateMessage, 5716, stated_elements| Vec<StatedElement>);

impl_type!(InteractiveElement, 80, element_id| i32, element_type_id| i32, enabled_skills| Vec<InteractiveElementSkillVariant>, disabled_skills| Vec<InteractiveElementSkillVariant>);
impl_type!(InteractiveElementNamedSkill, 220, base| InteractiveElementSkill, name_id| VarInt);
impl_type!(InteractiveElementSkill, 219, skill_id| VarInt, skill_instance_uid| i32);
impl_type!(InteractiveElementWithAgeBonus, 398, base| InteractiveElement, age_bonus| i16);
impl_type!(MapObstacle, 200, obstacle_cell_id| VarShort, state| i8);
impl_type!(StatedElement, 108, element_id| i32, element_cell_id| VarShort, element_state| VarInt);
