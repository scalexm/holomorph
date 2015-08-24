use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(ShortcutBarAddErrorMessage, 6227, error| i8);
impl_type!(ShortcutBarAddRequestMessage, 6225, bar_type| i8, shortcut| ShortcutVariant);
impl_type!(ShortcutBarContentMessage, 6231, bar_type| i8, shortcuts| Vec<ShortcutVariant>);
impl_type!(ShortcutBarRefreshMessage, 6229, bar_type| i8, shortcut| ShortcutVariant);
impl_type!(ShortcutBarRemovedMessage, 6224, bar_type| i8, slot| i8);
impl_type!(ShortcutBarRemoveErrorMessage, 6222, error| i8);
impl_type!(ShortcutBarRemoveRequestMessage, 6228, bar_type| i8, slot| i8);
impl_type!(ShortcutBarSwapErrorMessage, 6226, error| i8);
impl_type!(ShortcutBarSwapRequestMessage, 6230, bar_type| i8, first_slot| i8, second_slot| i8);

impl_type!(Shortcut, 369, slot| i8);
impl_type!(ShortcutEmote, 389, base| Shortcut, emote_id| i8);
impl_type!(ShortcutObject, 367, base| Shortcut);
impl_type!(ShortcutObjectItem, 371, base| ShortcutObject, item_u_i_d| i32, item_g_i_d| i32);
impl_type!(ShortcutObjectPreset, 370, base| ShortcutObject, preset_id| i8);
impl_type!(ShortcutSmiley, 388, base| Shortcut, smiley_id| i8);
impl_type!(ShortcutSpell, 368, base| Shortcut, spell_id| VarShort);
