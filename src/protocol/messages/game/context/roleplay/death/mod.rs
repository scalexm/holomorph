use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(GameRolePlayFreeSoulRequestMessage, 745);
impl_type!(GameRolePlayGameOverMessage, 746);
impl_type!(GameRolePlayPlayerLifeStatusMessage, 5996, state| i8);
impl_type!(WarnOnPermaDeathMessage, 6512, enable| bool);
