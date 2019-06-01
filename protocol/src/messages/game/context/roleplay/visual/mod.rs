use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6114)]
pub struct GameRolePlaySpellAnimMessage<'a> {
    #[protocol(var)]
    pub caster_id: u64,
    #[protocol(var)]
    pub target_cell_id: u16,
    #[protocol(var)]
    pub spell_id: u16,
    pub spell_level: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
