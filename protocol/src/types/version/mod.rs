use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 393)]
pub struct VersionExtended<'a> {
    pub base: Version<'a>,
    pub install: u8,
    pub technology: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 11)]
pub struct Version<'a> {
    pub major: u8,
    pub minor: u8,
    pub release: u8,
    pub revision: u32,
    pub patch: u8,
    pub build_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
