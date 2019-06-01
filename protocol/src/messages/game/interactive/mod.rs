pub mod meeting;
pub mod skill;
pub mod zaap;

use crate::types::game::interactive::InteractiveElement;
use crate::types::game::interactive::StatedElement;
use crate::variants::InteractiveElementVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5708)]
pub struct InteractiveElementUpdatedMessage<'a> {
    pub interactive_element: InteractiveElement<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5002)]
pub struct InteractiveMapUpdateMessage<'a> {
    pub interactive_elements: std::borrow::Cow<'a, [InteractiveElementVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6112)]
pub struct InteractiveUseEndedMessage<'a> {
    #[protocol(var)]
    pub elem_id: u32,
    #[protocol(var)]
    pub skill_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6384)]
pub struct InteractiveUseErrorMessage<'a> {
    #[protocol(var)]
    pub elem_id: u32,
    #[protocol(var)]
    pub skill_instance_uid: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5745)]
pub struct InteractiveUsedMessage<'a> {
    #[protocol(var)]
    pub entity_id: u64,
    #[protocol(var)]
    pub elem_id: u32,
    #[protocol(var)]
    pub skill_id: u16,
    #[protocol(var)]
    pub duration: u16,
    pub can_move: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5001)]
pub struct InteractiveUseRequestMessage<'a> {
    #[protocol(var)]
    pub elem_id: u32,
    #[protocol(var)]
    pub skill_instance_uid: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5716)]
pub struct StatedMapUpdateMessage<'a> {
    pub stated_elements: std::borrow::Cow<'a, [StatedElement<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5709)]
pub struct StatedElementUpdatedMessage<'a> {
    pub stated_element: StatedElement<'a>,
}
