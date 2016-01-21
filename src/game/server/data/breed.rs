use protocol::*;
use protocol::types::game::look::EntityLook;

#[derive(Queryable)]
pub struct BreedData {
    id: i16,
    male_look: EntityLook,
    female_look: EntityLook,
    spawn_map: i32,
}

impl BreedData {
    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn spawn_map(&self) -> i32 {
        self.spawn_map
    }

    pub fn male_look(&self) -> &EntityLook {
        &self.male_look
    }

    pub fn female_look(&self) -> &EntityLook {
        &self.female_look
    }
}

#[derive(Clone, Queryable)]
pub struct HeadData {
    id: i16,
    breed_id: i16,
    skin: i16,
    gender: bool,
}

impl HeadData {
    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn breed_id(&self) -> i16 {
        self.breed_id
    }

    pub fn skin(&self) -> i16 {
        self.skin
    }

    pub fn gender(&self) -> bool {
        self.gender
    }
}
