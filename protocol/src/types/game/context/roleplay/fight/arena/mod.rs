use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(ArenaRankInfos, 499, rank| VarShort, best_rank| VarShort, victory_count| VarShort, fightcount| VarShort);
