use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6740)]
pub struct AccountInformationsUpdateMessage<'a> {
    pub subscription_end_date: f64,
    pub unlimited_restat_end_date: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
