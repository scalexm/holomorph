use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use types::game::look::EntityLook;
impl_type!(PartyCompanionBaseInformations, 453, index_id| i8, companion_generic_id| i8, entity_look| EntityLook);
impl_type!(PartyCompanionMemberInformations, 452, base| PartyCompanionBaseInformations, initiative| VarShort, life_points| VarInt, max_life_points| VarInt, prospecting| VarShort, regen_rate| i8);
