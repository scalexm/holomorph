use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(IdolFightPreparationUpdateMessage, 6586, idol_source| i8, idols| Vec<IdolVariant>);
impl_type!(IdolListMessage, 6585, chosen_idols| Vec<VarShort>, party_chosen_idols| Vec<VarShort>, party_idols| Vec<PartyIdolVariant>);
impl_type!(IdolPartyLostMessage, 6580, idol_id| VarShort);
impl_type!(IdolPartyRefreshMessage, 6583, party_idol| PartyIdol);
impl_type!(IdolPartyRegisterRequestMessage, 6582, register| bool);
impl_type!(IdolSelectedMessage, 6581, activate| Flag, party| Flag, idol_id| VarShort);
impl_type!(IdolSelectErrorMessage, 6584, activate| Flag, party| Flag, reason| i8, idol_id| VarShort);
impl_type!(IdolSelectRequestMessage, 6587, activate| Flag, party| Flag, idol_id| VarShort);

impl_type!(Idol, 489, id| VarShort, xp_bonus_percent| VarShort, drop_bonus_percent| VarShort);
impl_type!(PartyIdol, 490, base| Idol, owners_ids| Vec<i32>);
