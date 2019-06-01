pub mod tax;

use crate::types::game::character::CharacterMinimalInformations;
use crate::variants::PlayerStatusVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 88)]
pub struct GuildMember<'a> {
    pub base: CharacterMinimalInformations<'a>,
    #[protocol(flag)]
    pub sex: bool,
    #[protocol(flag)]
    pub haven_bag_shared: bool,
    pub breed: i8,
    #[protocol(var)]
    pub rank: u16,
    #[protocol(var)]
    pub given_experience: u64,
    pub experience_given_percent: u8,
    #[protocol(var)]
    pub rights: u32,
    pub connected: u8,
    pub alignment_side: i8,
    pub hours_since_last_connection: u16,
    #[protocol(var)]
    pub mood_smiley_id: u16,
    pub account_id: u32,
    pub achievement_points: i32,
    pub status: PlayerStatusVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 87)]
pub struct GuildEmblem<'a> {
    #[protocol(var)]
    pub symbol_shape: u16,
    pub symbol_color: i32,
    pub background_shape: u8,
    pub background_color: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 498)]
pub struct HavenBagFurnitureInformation<'a> {
    #[protocol(var)]
    pub cell_id: u16,
    pub funiture_id: i32,
    pub orientation: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
