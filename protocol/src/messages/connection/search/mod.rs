use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6144)]
pub struct AcquaintanceSearchMessage<'a> {
    pub nickname: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6142)]
pub struct AcquaintanceServerListMessage<'a> {
    #[protocol(var_contents)]
    pub servers: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6143)]
pub struct AcquaintanceSearchErrorMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
