use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6267)]
pub struct TrustStatusMessage<'a> {
    #[protocol(flag)]
    pub trusted: bool,
    #[protocol(flag)]
    pub certified: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
