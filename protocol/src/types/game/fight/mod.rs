use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 186)]
pub struct ProtectedEntityWaitingForHelpInfo<'a> {
    pub time_left_before_fight: i32,
    pub wait_time_for_placement: i32,
    pub nb_position_for_defensors: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
