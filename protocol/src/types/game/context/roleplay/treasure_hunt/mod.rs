use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 468)]
pub struct TreasureHuntStepFollowDirection<'a> {
    pub base: TreasureHuntStep<'a>,
    pub direction: u8,
    #[protocol(var)]
    pub map_count: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 463)]
pub struct TreasureHuntStep<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 472)]
pub struct TreasureHuntStepFollowDirectionToHint<'a> {
    pub base: TreasureHuntStep<'a>,
    pub direction: u8,
    #[protocol(var)]
    pub npc_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 465)]
pub struct TreasureHuntStepDig<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 462)]
pub struct TreasureHuntStepFight<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 473)]
pub struct TreasureHuntFlag<'a> {
    pub map_id: f64,
    pub state: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 466)]
pub struct PortalInformation<'a> {
    pub portal_id: i32,
    pub area_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 461)]
pub struct TreasureHuntStepFollowDirectionToPOI<'a> {
    pub base: TreasureHuntStep<'a>,
    pub direction: u8,
    #[protocol(var)]
    pub poi_label_id: u16,
}
