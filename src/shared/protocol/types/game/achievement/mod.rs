use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(Achievement, 363, id| VarShort, finished_objective| Vec<AchievementObjective>, started_objectives| Vec<AchievementStartedObjective>);
impl_type!(AchievementObjective, 404, id| VarInt, max_value| VarShort);
impl_type!(AchievementRewardable, 412, id| VarShort, finishedlevel| i8);
impl_type!(AchievementStartedObjective, 402, base| AchievementObjective, value| VarShort);
