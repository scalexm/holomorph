use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(LockableChangeCodeMessage, 5666, code| String);
impl_type!(LockableCodeResultMessage, 5672, result| i8);
impl_type!(LockableShowCodeDialogMessage, 5740, change_or_use| bool, code_size| i8);
impl_type!(LockableStateUpdateAbstractMessage, 5671, locked| bool);
impl_type!(LockableStateUpdateHouseDoorMessage, 5668, base| LockableStateUpdateAbstractMessage, house_id| VarInt);
impl_type!(LockableStateUpdateStorageMessage, 5669, base| LockableStateUpdateAbstractMessage, map_id| i32, element_id| VarInt);
impl_type!(LockableUseCodeMessage, 5667, code| String);
