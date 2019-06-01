use crate::variants::ShortcutVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6227)]
pub struct ShortcutBarAddErrorMessage<'a> {
    pub error: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6706)]
pub struct ShortcutBarReplacedMessage<'a> {
    pub bar_type: u8,
    pub shortcut: ShortcutVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6222)]
pub struct ShortcutBarRemoveErrorMessage<'a> {
    pub error: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6230)]
pub struct ShortcutBarSwapRequestMessage<'a> {
    pub bar_type: u8,
    pub first_slot: u8,
    pub second_slot: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6231)]
pub struct ShortcutBarContentMessage<'a> {
    pub bar_type: u8,
    pub shortcuts: std::borrow::Cow<'a, [ShortcutVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6224)]
pub struct ShortcutBarRemovedMessage<'a> {
    pub bar_type: u8,
    pub slot: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6225)]
pub struct ShortcutBarAddRequestMessage<'a> {
    pub bar_type: u8,
    pub shortcut: ShortcutVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6229)]
pub struct ShortcutBarRefreshMessage<'a> {
    pub bar_type: u8,
    pub shortcut: ShortcutVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6228)]
pub struct ShortcutBarRemoveRequestMessage<'a> {
    pub bar_type: u8,
    pub slot: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6226)]
pub struct ShortcutBarSwapErrorMessage<'a> {
    pub error: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
