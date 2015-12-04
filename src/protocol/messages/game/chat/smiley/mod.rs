use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(ChatSmileyExtraPackListMessage, 6596, pack_ids| Vec<u8>);
impl_type!(ChatSmileyMessage, 801, entity_id| i32, smiley_id| VarShort, account_id| i32);
impl_type!(ChatSmileyRequestMessage, 800, smiley_id| VarShort);
impl_type!(LocalizedChatSmileyMessage, 6185, base| ChatSmileyMessage, cell_id| VarShort);
impl_type!(MoodSmileyRequestMessage, 6192, smiley_id| VarShort);
impl_type!(MoodSmileyResultMessage, 6196, result_code| i8, smiley_id| VarShort);
impl_type!(MoodSmileyUpdateMessage, 6388, account_id| i32, player_id| VarInt, smiley_id| VarShort);
