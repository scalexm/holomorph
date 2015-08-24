use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(FightDispellableEffectExtendedInformations, 208, action_id| VarShort, source_id| i32, effect| AbstractFightDispellableEffectVariant);
