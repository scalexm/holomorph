use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6563)]
pub struct DisplayNumericalValuePaddockMessage<'a> {
    pub ride_id: i32,
    pub value: i32,
    pub type_: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
