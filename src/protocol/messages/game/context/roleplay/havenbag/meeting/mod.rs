use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use types::game::character::CharacterMinimalInformations;
impl_type!(InviteInHavenBagClosedMessage, 6645, host_informations| CharacterMinimalInformations);
impl_type!(InviteInHavenBagMessage, 6642, guest_informations| CharacterMinimalInformations, accept| bool);
impl_type!(InviteInHavenBagOfferMessage, 6643, host_informations| CharacterMinimalInformations, time_left_before_cancel| VarInt);
impl_type!(TeleportHavenBagAnswerMessage, 6646, accept| bool);
impl_type!(TeleportHavenBagRequestMessage, 6647, guest_id| VarLong);
