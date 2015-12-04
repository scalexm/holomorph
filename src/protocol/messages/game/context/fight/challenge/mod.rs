use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(ChallengeInfoMessage, 6022, challenge_id| VarShort, target_id| i32, xp_bonus| VarInt, drop_bonus| VarInt);
impl_type!(ChallengeResultMessage, 6019, challenge_id| VarShort, success| bool);
impl_type!(ChallengeTargetsListMessage, 5613, target_ids| Vec<i32>, target_cells| Vec<i16>);
impl_type!(ChallengeTargetsListRequestMessage, 5614, challenge_id| VarShort);
impl_type!(ChallengeTargetUpdateMessage, 6123, challenge_id| VarShort, target_id| i32);
