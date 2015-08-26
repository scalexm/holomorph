use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::variants::MapCoordinatesVariant;
impl_type!(CompassResetMessage, 5584, type_| i8);
impl_type!(CompassUpdateMessage, 5591, type_| i8, coords| MapCoordinatesVariant);
impl_type!(CompassUpdatePartyMemberMessage, 5589, base| CompassUpdateMessage, member_id| VarInt);
impl_type!(CompassUpdatePvpSeekMessage, 6013, base| CompassUpdateMessage, member_id| VarInt, member_name| String);