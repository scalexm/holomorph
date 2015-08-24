use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::game::context::MapCoordinatesVariant;
impl_type!(CompassResetMessage, 5584, ttype| i8);
impl_type!(CompassUpdateMessage, 5591, ttype| i8, coords| MapCoordinatesVariant);
impl_type!(CompassUpdatePartyMemberMessage, 5589, base| CompassUpdateMessage, member_id| VarInt);
impl_type!(CompassUpdatePvpSeekMessage, 6013, base| CompassUpdateMessage, member_id| VarInt, member_name| String);
