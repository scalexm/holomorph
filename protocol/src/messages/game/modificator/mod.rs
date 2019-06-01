use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6493)]
pub struct AreaFightModificatorUpdateMessage<'a> {
    pub spell_pair_id: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
