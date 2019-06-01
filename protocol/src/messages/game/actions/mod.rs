pub mod fight;
pub mod sequence;

use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 957)]
pub struct GameActionAcknowledgementMessage<'a> {
    pub valid: bool,
    pub action_id: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1002)]
pub struct GameActionNoopMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1000)]
pub struct AbstractGameActionMessage<'a> {
    #[protocol(var)]
    pub action_id: u16,
    pub source_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1001)]
pub struct AbstractGameActionWithAckMessage<'a> {
    pub base: AbstractGameActionMessage<'a>,
    pub wait_ack_id: i16,
}
