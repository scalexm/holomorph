pub mod meeting;

use crate::types::game::guild::HavenBagFurnitureInformation;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6652)]
pub struct KickHavenBagRequestMessage<'a> {
    #[protocol(var)]
    pub guest_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6620)]
pub struct HavenBagPackListMessage<'a> {
    pub pack_ids: &'a [i8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6619)]
pub struct EditHavenBagCancelRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6636)]
pub struct EnterHavenBagRequestMessage<'a> {
    #[protocol(var)]
    pub haven_bag_owner: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6621)]
pub struct CloseHavenBagFurnitureSequenceRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6644)]
pub struct HavenBagDailyLoteryMessage<'a> {
    pub return_type: u8,
    pub game_action_id: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6634)]
pub struct HavenBagFurnituresMessage<'a> {
    pub furnitures_infos: std::borrow::Cow<'a, [HavenBagFurnitureInformation<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6638)]
pub struct ChangeHavenBagRoomRequestMessage<'a> {
    pub room_id: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6639)]
pub struct ChangeThemeRequestMessage<'a> {
    pub theme: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6632)]
pub struct EditHavenBagStartMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6630)]
pub struct RoomAvailableUpdateMessage<'a> {
    pub nb_room: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6626)]
pub struct EditHavenBagRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6637)]
pub struct HavenBagFurnituresRequestMessage<'a> {
    #[protocol(var_contents)]
    pub cell_ids: std::borrow::Cow<'a, [u16]>,
    pub funiture_ids: std::borrow::Cow<'a, [i32]>,
    pub orientations: &'a [u8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6628)]
pub struct EditHavenBagFinishedMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6631)]
pub struct ExitHavenBagRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6635)]
pub struct OpenHavenBagFurnitureSequenceRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
