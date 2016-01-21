use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(DungeonKeyRingMessage, 6299, availables| Vec<VarShort>, unavailables| Vec<VarShort>);
impl_type!(DungeonKeyRingUpdateMessage, 6296, dungeon_id| VarShort, available| bool);
