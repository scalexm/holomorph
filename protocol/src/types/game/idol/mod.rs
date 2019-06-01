use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 489)]
pub struct Idol<'a> {
    #[protocol(var)]
    pub id: u16,
    #[protocol(var)]
    pub xp_bonus_percent: u16,
    #[protocol(var)]
    pub drop_bonus_percent: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 490)]
pub struct PartyIdol<'a> {
    pub base: Idol<'a>,
    #[protocol(var_contents)]
    pub owners_ids: std::borrow::Cow<'a, [u64]>,
}
