use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(NotificationByServerMessage, 6103, id| VarShort, parameters| Vec<String>, force_open| bool);
impl_type!(NotificationListMessage, 6087, flags| Vec<VarInt>);
impl_type!(NotificationResetMessage, 6089);
impl_type!(NotificationUpdateFlagMessage, 6090, index| VarShort);
