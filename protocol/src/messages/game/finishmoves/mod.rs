use crate::types::game::finishmoves::FinishMoveInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6703)]
pub struct FinishMoveSetRequestMessage<'a> {
    pub finish_move_id: u32,
    pub finish_move_state: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6704)]
pub struct FinishMoveListMessage<'a> {
    pub finish_moves: std::borrow::Cow<'a, [FinishMoveInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6702)]
pub struct FinishMoveListRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
