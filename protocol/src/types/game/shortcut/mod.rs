use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(Shortcut, 369, slot| i8);
impl_type!(ShortcutEmote, 389, base| Shortcut, emote_id| i8);
impl_type!(ShortcutObject, 367, base| Shortcut);
impl_type!(ShortcutObjectIdolsPreset, 492, base| ShortcutObject, preset_id| i8);
impl_type!(ShortcutObjectItem, 371, base| ShortcutObject, item_uid| i32, item_gid| i32);
impl_type!(ShortcutObjectPreset, 370, base| ShortcutObject, preset_id| i8);
impl_type!(ShortcutSmiley, 388, base| Shortcut, smiley_id| VarShort);
impl_type!(ShortcutSpell, 368, base| Shortcut, spell_id| VarShort);
