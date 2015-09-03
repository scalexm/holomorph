use std::collections::HashMap;
use shared::protocol::*;
use shared::protocol::types::game::character::characteristic::CharacterBaseCharacteristic;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Field {
    Base,
    Additionnal,
    ObjectsAndMount,
    AlignGift,
    ContextModif,
}

pub struct Row {
    fields: HashMap<Field, f64>,
}

impl Row {
    pub fn new() -> Row {
        let mut fields = HashMap::new();
        let _ = fields.insert(Field::Base, 0.);
        let _ = fields.insert(Field::Additionnal, 0.);
        let _ = fields.insert(Field::ObjectsAndMount, 0.);
        let _ = fields.insert(Field::AlignGift, 0.);
        let _ = fields.insert(Field::ContextModif, 0.);

        Row {
            fields: fields,
        }
    }

    pub fn total(&self) -> i16 {
        (self.fields[&Field::Base] as i16)
            + (self.fields[&Field::Additionnal] as i16)
            + (self.fields[&Field::ObjectsAndMount] as i16)
            + (self.fields[&Field::AlignGift] as i16)
            + (self.fields[&Field::ContextModif] as i16)
    }

    pub fn add(&mut self, field: Field, val: f64) {
        *self.fields.get_mut(&field).unwrap() += val;
    }

    pub fn as_base_characteristic(&self) -> CharacterBaseCharacteristic {
        CharacterBaseCharacteristic {
            base: VarShort(self.fields[&Field::Base] as i16),
            additionnal: VarShort(self.fields[&Field::Additionnal] as i16),
            objects_and_mount_bonus: VarShort(self.fields[&Field::ObjectsAndMount] as i16),
            align_gift_bonus: VarShort(self.fields[&Field::AlignGift] as i16),
            context_modif: VarShort(self.fields[&Field::ContextModif] as i16),
        }
    }
}
