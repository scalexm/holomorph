use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(ObjectGroundAddedMessage, 3017, cell_id| VarShort, object_gid| VarShort);
impl_type!(ObjectGroundListAddedMessage, 5925, cells| Vec<VarShort>, reference_ids| Vec<VarShort>);
impl_type!(ObjectGroundRemovedMessage, 3014, cell| VarShort);
impl_type!(ObjectGroundRemovedMultipleMessage, 5944, cells| Vec<VarShort>);
