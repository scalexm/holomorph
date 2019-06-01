use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5675)]
pub struct DocumentReadingBeginMessage<'a> {
    #[protocol(var)]
    pub document_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6536)]
pub struct ComicReadingBeginMessage<'a> {
    #[protocol(var)]
    pub comic_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
