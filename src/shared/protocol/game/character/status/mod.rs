use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(PlayerStatusUpdateErrorMessage, 6385);
impl_type!(PlayerStatusUpdateMessage, 6386, account_id| i32, player_id| VarInt, status| PlayerStatusVariant);
impl_type!(PlayerStatusUpdateRequestMessage, 6387, status| PlayerStatusVariant);

impl_type!(PlayerStatus, 415, status_id| i8);
impl_type!(PlayerStatusExtended, 414, base| PlayerStatus, message| String);
