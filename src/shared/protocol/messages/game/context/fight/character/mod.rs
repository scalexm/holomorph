use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::variants::GameContextActorInformationsVariant; use protocol::variants::GameFightFighterInformationsVariant;
impl_type!(GameFightRefreshFighterMessage, 6309, informations| GameContextActorInformationsVariant);
impl_type!(GameFightShowFighterMessage, 5864, informations| GameFightFighterInformationsVariant);
impl_type!(GameFightShowFighterRandomStaticPoseMessage, 6218, base| GameFightShowFighterMessage);
