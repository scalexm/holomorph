use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::types::game::inventory::preset::PresetItem; use protocol::types::game::inventory::preset::Preset;
impl_type!(InventoryPresetDeleteMessage, 6169, preset_id| i8);
impl_type!(InventoryPresetDeleteResultMessage, 6173, preset_id| i8, code| i8);
impl_type!(InventoryPresetItemUpdateErrorMessage, 6211, code| i8);
impl_type!(InventoryPresetItemUpdateMessage, 6168, preset_id| i8, preset_item| PresetItem);
impl_type!(InventoryPresetItemUpdateRequestMessage, 6210, preset_id| i8, position| i8, obj_uid| VarInt);
impl_type!(InventoryPresetSaveCustomMessage, 6329, preset_id| i8, symbol_id| i8, items_positions| Vec<u8>, items_uids| Vec<VarInt>);
impl_type!(InventoryPresetSaveMessage, 6165, preset_id| i8, symbol_id| i8, save_equipment| bool);
impl_type!(InventoryPresetSaveResultMessage, 6170, preset_id| i8, code| i8);
impl_type!(InventoryPresetUpdateMessage, 6171, preset| Preset);
impl_type!(InventoryPresetUseMessage, 6167, preset_id| i8);
impl_type!(InventoryPresetUseResultMessage, 6163, preset_id| i8, code| i8, unlinked_position| Vec<u8>);