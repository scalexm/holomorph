use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(TeleportBuddiesAnswerMessage, 6294, accept| bool);
impl_type!(TeleportBuddiesMessage, 6289, dungeon_id| VarShort);
impl_type!(TeleportBuddiesRequestedMessage, 6302, dungeon_id| VarShort, inviter_id| VarLong, invalid_buddies_ids| Vec<VarLong>);
impl_type!(TeleportToBuddyAnswerMessage, 6293, dungeon_id| VarShort, buddy_id| VarLong, accept| bool);
impl_type!(TeleportToBuddyCloseMessage, 6303, dungeon_id| VarShort, buddy_id| VarLong);
impl_type!(TeleportToBuddyOfferMessage, 6287, dungeon_id| VarShort, buddy_id| VarLong, time_left| VarInt);
