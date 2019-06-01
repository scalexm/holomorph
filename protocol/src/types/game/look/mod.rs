use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 54)]
pub struct SubEntity<'a> {
    pub binding_point_category: u8,
    pub binding_point_index: u8,
    pub sub_entity_look: EntityLook<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 405)]
pub struct IndexedEntityLook<'a> {
    pub look: EntityLook<'a>,
    pub index: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 55)]
pub struct EntityLook<'a> {
    #[protocol(var)]
    pub bones_id: u16,
    #[protocol(var_contents)]
    pub skins: std::borrow::Cow<'a, [u16]>,
    pub indexed_colors: std::borrow::Cow<'a, [i32]>,
    #[protocol(var_contents)]
    pub scales: std::borrow::Cow<'a, [i16]>,
    pub subentities: Vec<SubEntity<'a>>,
}
