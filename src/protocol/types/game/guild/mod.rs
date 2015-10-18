pub mod tax;
use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use types::game::character::CharacterMinimalInformations; use variants::PlayerStatusVariant;
impl_type!(GuildEmblem, 87, symbol_shape| VarShort, symbol_color| i32, background_shape| i8, background_color| i32);
impl_type!(GuildMember, 88, base| CharacterMinimalInformations, breed| i8, sex| bool, rank| VarShort, given_experience| VarLong, experience_given_percent| i8, rights| VarInt, connected| i8, alignment_side| i8, hours_since_last_connection| i16, mood_smiley_id| VarShort, account_id| i32, achievement_points| i32, status| PlayerStatusVariant);
