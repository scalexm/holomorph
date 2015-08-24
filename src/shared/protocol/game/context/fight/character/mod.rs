use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::game::context::fight::GameFightFighterInformationsVariant; use protocol::game::context::GameContextActorInformationsVariant;
impl_type!(GameFightRefreshFighterMessage, 6309, informations| GameContextActorInformationsVariant);
impl_type!(GameFightShowFighterMessage, 5864, informations| GameFightFighterInformationsVariant);
impl_type!(GameFightShowFighterRandomStaticPoseMessage, 6218, base| GameFightShowFighterMessage);
