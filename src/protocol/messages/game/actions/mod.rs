pub mod sequence;
pub mod fight;
use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(AbstractGameActionMessage, 1000, action_id| VarShort, source_id| i32);
impl_type!(AbstractGameActionWithAckMessage, 1001, base| AbstractGameActionMessage, wait_ack_id| i16);
impl_type!(GameActionAcknowledgementMessage, 957, valid| bool, action_id| i8);
impl_type!(GameActionNoopMessage, 1002);
