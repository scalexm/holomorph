use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6053)]
pub struct CinematicMessage<'a> {
    #[protocol(var)]
    pub cinematic_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6266)]
pub struct URLOpenMessage<'a> {
    pub url_id: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
