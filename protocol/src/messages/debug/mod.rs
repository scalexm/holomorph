use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 2002)]
pub struct DebugClearHighlightCellsMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6028)]
pub struct DebugInClientMessage<'a> {
    pub level: u8,
    pub message: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 2001)]
pub struct DebugHighlightCellsMessage<'a> {
    pub color: f64,
    #[protocol(var_contents)]
    pub cells: std::borrow::Cow<'a, [u16]>,
}
