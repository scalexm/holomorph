use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6707)]
pub struct SpellVariantActivationRequestMessage<'a> {
    #[protocol(var)]
    pub spell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6705)]
pub struct SpellVariantActivationMessage<'a> {
    #[protocol(var)]
    pub spell_id: u16,
    pub result: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
