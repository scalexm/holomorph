use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(GameRolePlayArenaFightAnswerMessage, 6279, fight_id| i32, accept| bool);
impl_type!(GameRolePlayArenaFighterStatusMessage, 6281, fight_id| i32, player_id| i32, accepted| bool);
impl_type!(GameRolePlayArenaFightPropositionMessage, 6276, fight_id| i32, allies_id| Vec<i32>, duration| VarShort);
impl_type!(GameRolePlayArenaRegisterMessage, 6280, battle_mode| i32);
impl_type!(GameRolePlayArenaRegistrationStatusMessage, 6284, registered| bool, step| i8, battle_mode| i32);
impl_type!(GameRolePlayArenaSwitchToFightServerMessage, 6575, address| String, port| i16, ticket| VarIntVec<u8>);
impl_type!(GameRolePlayArenaSwitchToGameServerMessage, 6574, valid_token| bool, ticket| VarIntVec<u8>, home_server_id| i16);
impl_type!(GameRolePlayArenaUnregisterMessage, 6282);
impl_type!(GameRolePlayArenaUpdatePlayerInfosMessage, 6301, rank| VarShort, best_daily_rank| VarShort, best_rank| VarShort, victory_count| VarShort, arena_fightcount| VarShort);
