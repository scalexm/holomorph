use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(EmoteAddMessage, 5644, emote_id| i8);
impl_type!(EmoteListMessage, 5689, emote_ids| Vec<u8>);
impl_type!(EmotePlayAbstractMessage, 5690, emote_id| i8, emote_start_time| f64);
impl_type!(EmotePlayErrorMessage, 5688, emote_id| i8);
impl_type!(EmotePlayMassiveMessage, 5691, base| EmotePlayAbstractMessage, actor_ids| Vec<i32>);
impl_type!(EmotePlayMessage, 5683, base| EmotePlayAbstractMessage, actor_id| i32, account_id| i32);
impl_type!(EmotePlayRequestMessage, 5685, emote_id| i8);
impl_type!(EmoteRemoveMessage, 5687, emote_id| i8);
