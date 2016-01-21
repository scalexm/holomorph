use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(ActorAlignmentInformations, 201, alignment_side| i8, alignment_value| i8, alignment_grade| i8, character_power| f64);
impl_type!(ActorExtendedAlignmentInformations, 202, base| ActorAlignmentInformations, honor| VarShort, honor_grade_floor| VarShort, honor_next_grade_floor| VarShort, aggressable| i8);
