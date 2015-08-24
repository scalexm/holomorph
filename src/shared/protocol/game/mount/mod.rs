use std::io::{Read, Write};
use io::Result;
use protocol::*;
use protocol::game::data::items::effects::ObjectEffectInteger; 
impl_type!(ItemDurability, 168, durability| i16, durability_max| i16);
impl_type!(MountClientData, 178, sex| Flag, is_rideable| Flag, is_wild| Flag, is_fecondation_ready| Flag, id| f64, model| VarInt, ancestor| Vec<i32>, behaviors| Vec<i32>, name| String, owner_id| i32, experience| VarLong, experience_for_level| VarLong, experience_for_next_level| f64, level| i8, max_pods| VarInt, stamina| VarInt, stamina_max| VarInt, maturity| VarInt, maturity_for_adult| VarInt, energy| VarInt, energy_max| VarInt, serenity| i32, aggressivity_max| i32, serenity_max| VarInt, love| VarInt, love_max| VarInt, fecondation_time| i32, boost_limiter| i32, boost_max| f64, reproduction_count| i32, reproduction_count_max| VarInt, effect_list| Vec<ObjectEffectInteger>);
impl_type!(UpdateMountBoost, 356, ttype| i8);
impl_type!(UpdateMountIntBoost, 357, base| UpdateMountBoost, value| i32);
