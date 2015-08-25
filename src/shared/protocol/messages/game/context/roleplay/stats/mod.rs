use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(StatsUpgradeRequestMessage, 5610, use_additionnal| bool, stat_id| i8, boost_point| VarShort);
impl_type!(StatsUpgradeResultMessage, 5609, result| i8, nb_charac_boost| VarShort);
