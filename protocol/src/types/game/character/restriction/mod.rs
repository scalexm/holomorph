use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 204)]
pub struct ActorRestrictionsInformations<'a> {
    #[protocol(flag)]
    pub cant_be_aggressed: bool,
    #[protocol(flag)]
    pub cant_be_challenged: bool,
    #[protocol(flag)]
    pub cant_trade: bool,
    #[protocol(flag)]
    pub cant_be_attacked_by_mutant: bool,
    #[protocol(flag)]
    pub cant_run: bool,
    #[protocol(flag)]
    pub force_slow_walk: bool,
    #[protocol(flag)]
    pub cant_minimize: bool,
    #[protocol(flag)]
    pub cant_move: bool,
    #[protocol(flag)]
    pub cant_aggress: bool,
    #[protocol(flag)]
    pub cant_challenge: bool,
    #[protocol(flag)]
    pub cant_exchange: bool,
    #[protocol(flag)]
    pub cant_attack: bool,
    #[protocol(flag)]
    pub cant_chat: bool,
    #[protocol(flag)]
    pub cant_be_merchant: bool,
    #[protocol(flag)]
    pub cant_use_object: bool,
    #[protocol(flag)]
    pub cant_use_tax_collector: bool,
    #[protocol(flag)]
    pub cant_use_interactive: bool,
    #[protocol(flag)]
    pub cant_speak_to_npc: bool,
    #[protocol(flag)]
    pub cant_change_zone: bool,
    #[protocol(flag)]
    pub cant_attack_monster: bool,
    #[protocol(flag)]
    pub cant_walk8_directions: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
