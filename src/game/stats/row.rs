use std::collections::HashMap;
use protocol::*;
use protocol::types::game::character::characteristic::CharacterBaseCharacteristic;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Field {
    Base,
    Additionnal,
    ObjectsAndMount,
    AlignGift,
    ContextModif,
}

lazy_static! {
    static ref FHMAP: HashMap<Field, f64> = {
        let mut fields = HashMap::new();
        let _ = fields.insert(Field::Base, 0.);
        let _ = fields.insert(Field::Additionnal, 0.);
        let _ = fields.insert(Field::ObjectsAndMount, 0.);
        let _ = fields.insert(Field::AlignGift, 0.);
        let _ = fields.insert(Field::ContextModif, 0.);
        fields
    };
}

#[derive(Clone)]
pub struct Row {
    fields: HashMap<Field, f64>,
}

fn balance(s1: f64, s2: f64) -> (i16, i16) {
    let diff = ((s1 + s2) as i16) - (s1 as i16) - (s2 as i16);
    (s1 as i16, (s2 as i16) + diff)
}

impl Row {
    pub fn new() -> Self {
        Row {
            fields: FHMAP.clone(),
        }
    }

    pub fn get(&self, field: Field) -> i16 {
        self.fields[&field] as i16
    }

    pub fn total(&self) -> i16 {
        (self.fields[&Field::Base]
            + self.fields[&Field::Additionnal]) as i16
            + (self.fields[&Field::ObjectsAndMount]
            + self.fields[&Field::AlignGift]
            + self.fields[&Field::ContextModif]) as i16
    }

    pub fn add(&mut self, field: Field, val: f64) {
        *self.fields.get_mut(&field).unwrap() += val;
    }

    pub fn as_base_characteristic(&self) -> CharacterBaseCharacteristic {
        let additionnal = self.fields[&Field::Additionnal];
        let base = self.fields[&Field::Base];

        let obj = self.fields[&Field::ObjectsAndMount];
        let align = self.fields[&Field::AlignGift];
        let context = self.fields[&Field::ContextModif];

        let (total_base, total_bonus) = balance(additionnal + base, obj + align + context);

        CharacterBaseCharacteristic {
            base: VarShort(total_base - (additionnal as i16)),
            additionnal: VarShort(additionnal as i16),
            objects_and_mount_bonus: VarShort(total_bonus - (align as i16) - (context as i16)),
            align_gift_bonus: VarShort(align as i16),
            context_modif: VarShort(context as i16),
        }
    }
}
