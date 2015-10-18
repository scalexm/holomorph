use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(ProtectedEntityWaitingForHelpInfo, 186, time_left_before_fight| i32, wait_time_for_placement| i32, nb_position_for_defensors| i8);
