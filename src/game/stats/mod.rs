pub mod row;

use std::collections::HashMap;
use self::row::{Row, Field};
use protocol::types::game::character::characteristic::CharacterBaseCharacteristic;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Type {
    Initiative,
    Prospecting,
    ActionPoints,
    MovementPoints,
    Strength,
    Vitality,
    Wisdom,
    Chance,
    Agility,
    Intelligence,
    Range,
    SummonableCreaturesBoost,
    Reflect,
    CriticalHit,
    CriticalMiss,
    HealBonus,
    AllDamagesBonus,
    WeaponDamagesBonusPercent,
    DamagesBonusPercent,
    TrapBonus,
    TrapBonusPercent,
    GlyphBonusPercent,
    PermanentDamagePercent,
    TackleBlock,
    TackleEvade,
    PAAttack,
    PMAttack,
    PushDamageBonus,
    CriticalDamageBonus,
    NeutralDamageBonus,
    EarthDamageBonus,
    WaterDamageBonus,
    AirDamageBonus,
    FireDamageBonus,
    DodgePALostProbability,
    DodgePMLostProbability,
    NeutralElementResistPercent,
    EarthElementResistPercent,
    WaterElementResistPercent,
    AirElementResistPercent,
    FireElementResistPercent,
    NeutralElementReduction,
    EarthElementReduction,
    WaterElementReduction,
    AirElementReduction,
    FireElementReduction,
    PushDamageReduction,
    CriticalDamageReduction,
    PvpNeutralElementResistPercent,
    PvpEarthElementResistPercent,
    PvpWaterElementResistPercent,
    PvpAirElementResistPercent,
    PvpFireElementResistPercent,
    PvpNeutralElementReduction,
    PvpEarthElementReduction,
    PvpWaterElementReduction,
    PvpAirElementReduction,
    PvpFireElementReduction,
}

macro_rules! fill_stats {
    ($($x: ident),*) => {{
        let mut rows = HashMap::new();
        $(
            let _ = rows.insert(Type::$x, Row::new());
        )*
        rows
    }};
}

lazy_static! {
    static ref RHMAP: HashMap<Type, Row> = fill_stats!(
        Initiative,
        Prospecting,
        ActionPoints,
        MovementPoints,
        Strength,
        Vitality,
        Wisdom,
        Chance,
        Agility,
        Intelligence,
        Range,
        SummonableCreaturesBoost,
        Reflect,
        CriticalHit,
        CriticalMiss,
        HealBonus,
        AllDamagesBonus,
        WeaponDamagesBonusPercent,
        DamagesBonusPercent,
        TrapBonus,
        TrapBonusPercent,
        GlyphBonusPercent,
        PermanentDamagePercent,
        TackleBlock,
        TackleEvade,
        PAAttack,
        PMAttack,
        PushDamageBonus,
        CriticalDamageBonus,
        NeutralDamageBonus,
        EarthDamageBonus,
        WaterDamageBonus,
        AirDamageBonus,
        FireDamageBonus,
        DodgePALostProbability,
        DodgePMLostProbability,
        NeutralElementResistPercent,
        EarthElementResistPercent,
        WaterElementResistPercent,
        AirElementResistPercent,
        FireElementResistPercent,
        NeutralElementReduction,
        EarthElementReduction,
        WaterElementReduction,
        AirElementReduction,
        FireElementReduction,
        PushDamageReduction,
        CriticalDamageReduction,
        PvpNeutralElementResistPercent,
        PvpEarthElementResistPercent,
        PvpWaterElementResistPercent,
        PvpAirElementResistPercent,
        PvpFireElementResistPercent,
        PvpNeutralElementReduction,
        PvpEarthElementReduction,
        PvpWaterElementReduction,
        PvpAirElementReduction,
        PvpFireElementReduction
    );
}

pub struct List {
    rows: HashMap<Type, Row>,
}

impl List {
    pub fn new() -> Self {
        List {
            rows: RHMAP.clone(),
        }
    }

    pub fn add(&mut self, ty: Type, field: Field, val: i16) {
        let val = val as f64;
        match ty {
            Type::Wisdom => {
                let val = val / 10.;
                self.rows.get_mut(&Type::DodgePALostProbability)
                         .unwrap()
                         .add(field, val);
                self.rows.get_mut(&Type::DodgePMLostProbability)
                         .unwrap()
                         .add(field, val);
                self.rows.get_mut(&Type::PAAttack)
                         .unwrap()
                         .add(field, val);
                self.rows.get_mut(&Type::PMAttack)
                         .unwrap()
                         .add(field, val);
            }

            Type::Chance => {
                self.rows.get_mut(&Type::Initiative)
                         .unwrap()
                         .add(field, val);
                self.rows.get_mut(&Type::Prospecting)
                         .unwrap()
                         .add(field, val / 10.);
            }

            Type::Agility => {
                self.rows.get_mut(&Type::Initiative)
                         .unwrap()
                         .add(field, val);
                let val = val / 10.;
                self.rows.get_mut(&Type::TackleBlock)
                         .unwrap()
                         .add(field, val);
                self.rows.get_mut(&Type::TackleEvade)
                         .unwrap()
                         .add(field, val);
            }

            Type::Intelligence => {
                self.rows.get_mut(&Type::Initiative)
                         .unwrap()
                         .add(field, val);
            }

            Type::Strength => {
                self.rows.get_mut(&Type::Initiative)
                         .unwrap()
                         .add(field, val);
            }

            _ => (),
        }
        self.rows.get_mut(&ty).unwrap().add(field, val);
    }

    pub fn total(&self, ty: Type) -> i16 {
        self.rows[&ty].total()
    }

    pub fn get(&self, ty: Type, field: Field) -> i16 {
        self.rows[&ty].get(field)
    }

    pub fn as_base_characteristic(&self, ty: Type) -> CharacterBaseCharacteristic {
        self.rows[&ty].as_base_characteristic()
    }
}
