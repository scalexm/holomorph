use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(TeleportDestinationsListMessage, 5960, teleporter_type| i8, map_ids| Vec<i32>, sub_area_ids| Vec<VarShort>, costs| Vec<VarShort>, dest_teleporter_type| Vec<u8>);
impl_type!(TeleportRequestMessage, 5961, teleporter_type| i8, map_id| i32);
impl_type!(ZaapListMessage, 1604, base| TeleportDestinationsListMessage, spawn_map_id| i32);
impl_type!(ZaapRespawnSaveRequestMessage, 6572);
impl_type!(ZaapRespawnUpdatedMessage, 6571, map_id| i32);
