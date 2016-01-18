use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(PortalInformation, 466, portal_id| i32, area_id| i16);
impl_type!(TreasureHuntFlag, 473, map_id| i32, state| i8);
impl_type!(TreasureHuntStep, 463);
impl_type!(TreasureHuntStepDig, 465);
impl_type!(TreasureHuntStepFight, 462);
impl_type!(TreasureHuntStepFollowDirection, 468, base| TreasureHuntStep, direction| i8, map_count| VarShort);
impl_type!(TreasureHuntStepFollowDirectionToHint, 472, base| TreasureHuntStep, direction| i8, npc_id| VarShort);
impl_type!(TreasureHuntStepFollowDirectionToPOI, 461, base| TreasureHuntStep, direction| i8, poi_label_id| VarShort);
