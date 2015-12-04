use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(AbstractFightDispellableEffect, 206, uid| VarInt, target_id| i32, turn_duration| i16, dispelable| i8, spell_id| VarShort, effect_id| VarInt, parent_boost_uid| VarInt);
impl_type!(FightTemporaryBoostEffect, 209, base| AbstractFightDispellableEffect, delta| i16);
impl_type!(FightTemporaryBoostStateEffect, 214, base| FightTemporaryBoostEffect, state_id| i16);
impl_type!(FightTemporaryBoostWeaponDamagesEffect, 211, base| FightTemporaryBoostEffect, weapon_type_id| i16);
impl_type!(FightTemporarySpellBoostEffect, 207, base| FightTemporaryBoostEffect, boosted_spell_id| VarShort);
impl_type!(FightTemporarySpellImmunityEffect, 366, base| AbstractFightDispellableEffect, immune_spell_id| i32);
impl_type!(FightTriggeredEffect, 210, base| AbstractFightDispellableEffect, param1| i32, param2| i32, param3| i32, delay| i16);
impl_type!(GameActionMark, 351, mark_author_id| i32, mark_team_id| i8, mark_spell_id| i32, mark_spell_level| i8, mark_id| i16, mark_type| i8, markimpact_cell| i16, cells| Vec<GameActionMarkedCell>, active| bool);
impl_type!(GameActionMarkedCell, 85, cell_id| VarShort, zone_size| i8, cell_color| i32, cells_type| i8);
