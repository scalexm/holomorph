use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 377)]
pub struct TrustCertificate<'a> {
    pub id: u32,
    pub hash: &'a str,
}
