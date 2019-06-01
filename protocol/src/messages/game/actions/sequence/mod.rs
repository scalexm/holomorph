use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 956)]
pub struct SequenceEndMessage<'a> {
    #[protocol(var)]
    pub action_id: u16,
    pub author_id: f64,
    pub sequence_type: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 955)]
pub struct SequenceStartMessage<'a> {
    pub sequence_type: i8,
    pub author_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
