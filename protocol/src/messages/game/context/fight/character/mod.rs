use crate::variants::GameContextActorInformationsVariant;
use crate::variants::GameFightFighterInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5864)]
pub struct GameFightShowFighterMessage<'a> {
    pub informations: GameFightFighterInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6218)]
pub struct GameFightShowFighterRandomStaticPoseMessage<'a> {
    pub base: GameFightShowFighterMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6309)]
pub struct GameFightRefreshFighterMessage<'a> {
    pub informations: GameContextActorInformationsVariant<'a>,
}
