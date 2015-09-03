use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(ActorRestrictionsInformations, 204, cant_be_aggressed| Flag, cant_be_challenged| Flag, cant_trade| Flag, cant_be_attacked_by_mutant| Flag, cant_run| Flag, force_slow_walk| Flag, cant_minimize| Flag, cant_move| Flag, cant_aggress| Flag, cant_challenge| Flag, cant_exchange| Flag, cant_attack| Flag, cant_chat| Flag, cant_be_merchant| Flag, cant_use_object| Flag, cant_use_tax_collector| Flag, cant_use_interactive| Flag, cant_speak_to_npc| Flag, cant_change_zone| Flag, cant_attack_monster| Flag, cant_walk8_directions| Flag);
