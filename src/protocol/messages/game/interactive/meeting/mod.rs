use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(TeleportBuddiesAnswerMessage, 6294, accept| bool);
impl_type!(TeleportBuddiesMessage, 6289, dungeon_id| VarShort);
impl_type!(TeleportBuddiesRequestedMessage, 6302, dungeon_id| VarShort, inviter_id| VarInt, invalid_buddies_ids| Vec<VarInt>);
impl_type!(TeleportToBuddyAnswerMessage, 6293, dungeon_id| VarShort, buddy_id| VarInt, accept| bool);
impl_type!(TeleportToBuddyCloseMessage, 6303, dungeon_id| VarShort, buddy_id| VarInt);
impl_type!(TeleportToBuddyOfferMessage, 6287, dungeon_id| VarShort, buddy_id| VarInt, time_left| VarInt);
