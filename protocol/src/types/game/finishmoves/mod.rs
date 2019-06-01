use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 506)]
pub struct FinishMoveInformations<'a> {
    pub finish_move_id: u32,
    pub finish_move_state: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
