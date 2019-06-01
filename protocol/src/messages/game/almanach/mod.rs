use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6341)]
pub struct AlmanachCalendarDateMessage<'a> {
    pub date: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
