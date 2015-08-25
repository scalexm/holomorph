use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(SkillActionDescription, 102, skill_id| VarShort);
impl_type!(SkillActionDescriptionCollect, 99, base| SkillActionDescriptionTimed, min| VarShort, max| VarShort);
impl_type!(SkillActionDescriptionCraft, 100, base| SkillActionDescription, probability| i8);
impl_type!(SkillActionDescriptionTimed, 103, base| SkillActionDescription, time| i8);
