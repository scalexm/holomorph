use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(IdolsPreset, 491, preset_id| i8, symbol_id| i8, idol_id| Vec<VarShort>);
impl_type!(Preset, 355, preset_id| i8, symbol_id| i8, mount| bool, objects| Vec<PresetItem>);
impl_type!(PresetItem, 354, position| i8, obj_gid| VarShort, obj_uid| VarInt);
