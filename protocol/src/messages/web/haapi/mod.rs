use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6767)]
pub struct HaapiTokenMessage<'a> {
    pub token: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6769)]
pub struct HaapiSessionMessage<'a> {
    pub key: &'a str,
    pub type_: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6649)]
pub struct HaapiApiKeyMessage<'a> {
    pub token: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6648)]
pub struct HaapiApiKeyRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6768)]
pub struct HaapiAuthErrorMessage<'a> {
    pub type_: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6766)]
pub struct HaapiTokenRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
