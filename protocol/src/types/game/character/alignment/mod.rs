use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 201)]
pub struct ActorAlignmentInformations<'a> {
    pub alignment_side: i8,
    pub alignment_value: u8,
    pub alignment_grade: u8,
    pub character_power: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 202)]
pub struct ActorExtendedAlignmentInformations<'a> {
    pub base: ActorAlignmentInformations<'a>,
    #[protocol(var)]
    pub honor: u16,
    #[protocol(var)]
    pub honor_grade_floor: u16,
    #[protocol(var)]
    pub honor_next_grade_floor: u16,
    pub aggressable: u8,
}
