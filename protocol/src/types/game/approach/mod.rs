use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 429)]
pub struct ServerSessionConstantLong<'a> {
    pub base: ServerSessionConstant<'a>,
    pub value: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 436)]
pub struct ServerSessionConstantString<'a> {
    pub base: ServerSessionConstant<'a>,
    pub value: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 430)]
pub struct ServerSessionConstant<'a> {
    #[protocol(var)]
    pub id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 433)]
pub struct ServerSessionConstantInteger<'a> {
    pub base: ServerSessionConstant<'a>,
    pub value: i32,
}
