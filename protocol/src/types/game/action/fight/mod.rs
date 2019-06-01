use crate::variants::AbstractFightDispellableEffectVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 208)]
pub struct FightDispellableEffectExtendedInformations<'a> {
    #[protocol(var)]
    pub action_id: u16,
    pub source_id: f64,
    pub effect: AbstractFightDispellableEffectVariant<'a>,
}
