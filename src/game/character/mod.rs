use shared::protocol::types::game::look::EntityLook;
use shared::protocol::*;
use shared::protocol::types::game::character::*;
use shared::protocol::types::game::character::choice::*;
use shared::protocol::types::game::character::characteristic::*;
use shared::protocol::types::game::character::alignment::*;
use shared::protocol::types::game::character::restriction::*;
use shared::protocol::types::game::context::roleplay::*;
use shared::protocol::types::game::context::*;
use std::io::Cursor;
use postgres::rows::Row;
use postgres;
use stats::{self, Type};
use stats::row::Field;

#[derive(Clone)]
pub struct CharacterMinimal {
    id: i32,
    account_id: i32,
    account_nickname: String,
    level: i16,
    name: String,
    breed: i8,
    sex: bool,
    look: EntityLook,
}

impl CharacterMinimal {
    pub fn from_sql<'a>(row: Row<'a>) -> (i32, Self) {
        let id = row.get("id");
        let breed: i16 = row.get("breed");

        let buf: Vec<u8> = row.get("look");
        let mut buf = Cursor::new(buf);
        let look = match EntityLook::deserialize(&mut buf) {
            Ok(look) => look,
            Err(_) => {
                panic!("EntityLook::deserialize failed while constructing character {}", id);
            }
        };

        (id, CharacterMinimal {
            id: id,
            account_id: row.get("account_id"),
            account_nickname: row.get("account_nickname"),
            level: row.get("level"),
            name: row.get("name"),
            breed: breed as i8,
            sex: row.get("sex"),
            look: look,
        })
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn account_id(&self) -> i32 {
        self.account_id
    }

    pub fn as_character_base(&self) -> CharacterBaseInformations {
        CharacterBaseInformations {
            base: CharacterMinimalPlusLookInformations {
                base: CharacterMinimalInformations {
                    base: AbstractCharacterInformation {
                        id: VarInt(self.id),
                    },
                    level: self.level as i8,
                    name: self.name.clone(),
                },
                entity_look: self.look.clone(),
            },
            breed: self.breed,
            sex: self.sex,
        }
    }
}

pub struct Character {
    base: CharacterMinimal,
    xp: i64,
    kamas: i32,
    stats_points: i16,
    additionnal_points: i16,
    spells_points: i16,
    energy_points: i16,
    stats: stats::List,
    map_id: i32,
    cell_id: i16,
}

#[derive(Debug)]
struct CellError(i16);

impl ::std::error::Error for CellError {
    fn description(&self) -> &str {
        "invalid cell"
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        None
    }
}

impl ::std::fmt::Display for CellError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "invalid cell: {}", self.0)
    }
}

impl Character {
    pub fn from_sql<'a>(base: CharacterMinimal, row: Row<'a>)
        -> postgres::Result<Character> {

        let mut stats = stats::List::new();

        stats.add(Type::Initiative, Field::Base, 1000);
        stats.add(Type::Prospecting, Field::Base, 100);
        stats.add(Type::ActionPoints, Field::Base, if base.level > 99 { 7 } else { 6 });
        stats.add(Type::MovementPoints, Field::Base, 3);
        stats.add(Type::SummonableCreaturesBoost, Field::Base, 1);

        stats.add(Type::Vitality, Field::Base, try!(row.get_opt("base_vitality")));
        stats.add(Type::Wisdom, Field::Base, try!(row.get_opt("base_wisdom")));
        stats.add(Type::Strength, Field::Base, try!(row.get_opt("base_strength")));
        stats.add(Type::Intelligence, Field::Base, try!(row.get_opt("base_intelligence")));
        stats.add(Type::Chance, Field::Base, try!(row.get_opt("base_chance")));
        stats.add(Type::Agility, Field::Base, try!(row.get_opt("base_agility")));

        stats.add(Type::Vitality, Field::Additionnal,
            try!(row.get_opt("additionnal_vitality")));
        stats.add(Type::Wisdom, Field::Additionnal,
            try!(row.get_opt("additionnal_wisdom")));
        stats.add(Type::Strength, Field::Additionnal,
            try!(row.get_opt("additionnal_strength")));
        stats.add(Type::Intelligence, Field::Additionnal,
            try!(row.get_opt("additionnal_intelligence")));
        stats.add(Type::Chance, Field::Additionnal,
            try!(row.get_opt("additionnal_chance")));
        stats.add(Type::Agility, Field::Additionnal,
            try!(row.get_opt("additionnal_agility")));

        let cell_id: i16 = try!(row.get_opt("cell_id"));
        if cell_id < 0 || cell_id > 560 {
            return Err(postgres::error::Error::Conversion(Box::new(CellError(cell_id))));
        }

        Ok(Character {
            base: base,
            xp: try!(row.get_opt("xp")),
            kamas: try!(row.get_opt("kamas")),
            stats_points: try!(row.get_opt("stats_points")),
            additionnal_points: try!(row.get_opt("additionnal_points")),
            spells_points: try!(row.get_opt("spells_points")),
            energy_points: try!(row.get_opt("energy_points")),
            stats: stats,
            map_id: try!(row.get_opt("map_id")),
            cell_id: cell_id,
        })
    }

    pub fn map_id(&self) -> i32 {
        self.map_id
    }

    pub fn set_map_id(&mut self, map_id: i32) {
        self.map_id = map_id;
    }

    pub fn cell_id(&self) -> i16 {
        self.cell_id
    }

    pub fn kamas(&self) -> i32 {
        self.kamas
    }

    pub fn minimal(&self) -> &CharacterMinimal {
        &self.base
    }

    pub fn max_life(&self) ->i32 {
        let mut life = 50 + (self.base.level - 1) * 10;
        if self.base.level > 100 {
            life -= (self.base.level - 100) * 5;
        }
        (life as i32) + (self.stats.total(Type::Vitality) as i32)
    }

    pub fn get_alignment_informations(&self) -> ActorAlignmentInformations {
        ActorAlignmentInformations {
            alignment_side: 0,
            alignment_value: 0,
            alignment_grade: 0,
            character_power: VarInt(0),
        }
    }

    pub fn as_actor(&self) -> GameRolePlayCharacterInformations {
        GameRolePlayCharacterInformations {
            base: GameRolePlayHumanoidInformations {
                base: GameRolePlayNamedActorInformations {
                    base: GameRolePlayActorInformations {
                        base: GameContextActorInformations {
                            contextual_id: self.base.id,
                            look: self.base.look.clone(),
                            disposition: EntityDispositionInformations {
                                cell_id: self.cell_id,
                                direction: 0,
                            }.into(),
                        },
                    },
                    name: self.base.name.clone(),
                },
                humanoid_info: HumanInformations {
                    restrictions: ActorRestrictionsInformations {
                        cant_be_aggressed: Flag(false),
                        cant_be_challenged: Flag(false),
                        cant_trade: Flag(false),
                        cant_be_attacked_by_mutant: Flag(false),
                        cant_run: Flag(false),
                        force_slow_walk: Flag(false),
                        cant_minimize: Flag(false),
                        cant_move: Flag(false),
                        cant_aggress: Flag(false),
                        cant_challenge: Flag(false),
                        cant_exchange: Flag(false),
                        cant_attack: Flag(false),
                        cant_chat: Flag(false),
                        cant_be_merchant: Flag(false),
                        cant_use_object: Flag(false),
                        cant_use_tax_collector: Flag(false),
                        cant_use_interactive: Flag(false),
                        cant_speak_to_npc: Flag(false),
                        cant_change_zone: Flag(false),
                        cant_attack_monster: Flag(false),
                        cant_walk8_directions: Flag(false),
                    },
                    sex: self.base.sex,
                    options: Vec::new(),
                }.into(),
                account_id: self.base.account_id,
            },
            alignment_infos: self.get_alignment_informations(),
        }
    }

    pub fn get_character_characteristics(&self) -> CharacterCharacteristicsInformations {
        CharacterCharacteristicsInformations {
            experience: VarLong(self.xp),
            experience_level_floor: VarLong(self.xp),
            experience_next_level_floor: VarLong(self.xp + 1),
            kamas: self.kamas,
            stats_points: VarShort(self.stats_points),
            additionnal_points: VarShort(self.additionnal_points),
            spells_points: VarShort(self.spells_points),

            alignment_infos: ActorExtendedAlignmentInformations {
                base: self.get_alignment_informations(),
                honor: VarShort(0),
                honor_grade_floor: VarShort(0),
                honor_next_grade_floor: VarShort(0),
                aggressable: 0,
            },

            life_points: VarInt(self.max_life()),
            max_life_points: VarInt(self.max_life()),
            energy_points: VarShort(self.energy_points),
            max_energy_points: VarShort(10000),
            action_points_current: VarShort(self.stats.total(Type::ActionPoints)),
            movement_points_current: VarShort(self.stats.total(Type::MovementPoints)),

            initiative: self.stats.as_base_characteristic(Type::Initiative),
            prospecting: self.stats.as_base_characteristic(Type::Prospecting),
            action_points: self.stats.as_base_characteristic(Type::ActionPoints),
            movement_points: self.stats.as_base_characteristic(Type::MovementPoints),
            strength: self.stats.as_base_characteristic(Type::Strength),
            vitality: self.stats.as_base_characteristic(Type::Vitality),
            wisdom: self.stats.as_base_characteristic(Type::Wisdom),
            chance: self.stats.as_base_characteristic(Type::Chance),
            agility: self.stats.as_base_characteristic(Type::Agility),
            intelligence: self.stats.as_base_characteristic(Type::Intelligence),
            range: self.stats.as_base_characteristic(Type::Range),
            summonable_creatures_boost:
                self.stats.as_base_characteristic(Type::SummonableCreaturesBoost),
            reflect: self.stats.as_base_characteristic(Type::Reflect),
            critical_hit: self.stats.as_base_characteristic(Type::CriticalHit),
            critical_hit_weapon: VarShort(50),
            critical_miss: self.stats.as_base_characteristic(Type::CriticalMiss),
            heal_bonus: self.stats.as_base_characteristic(Type::HealBonus),
            all_damages_bonus:
                self.stats.as_base_characteristic(Type::AllDamagesBonus),
            weapon_damages_bonus_percent:
                self.stats.as_base_characteristic(Type::WeaponDamagesBonusPercent),
            damages_bonus_percent:
                self.stats.as_base_characteristic(Type::DamagesBonusPercent),
            trap_bonus: self.stats.as_base_characteristic(Type::TrapBonus),
            trap_bonus_percent:
                self.stats.as_base_characteristic(Type::TrapBonusPercent),
            glyph_bonus_percent:
                self.stats.as_base_characteristic(Type::GlyphBonusPercent),
            permanent_damage_percent:
                self.stats.as_base_characteristic(Type::PermanentDamagePercent),
            tackle_block: self.stats.as_base_characteristic(Type::TackleBlock),
            tackle_evade: self.stats.as_base_characteristic(Type::TackleEvade),
            pa_attack: self.stats.as_base_characteristic(Type::PAAttack),
            pm_attack: self.stats.as_base_characteristic(Type::PMAttack),
            push_damage_bonus:
                self.stats.as_base_characteristic(Type::PushDamageBonus),
            critical_damage_bonus:
                self.stats.as_base_characteristic(Type::CriticalDamageBonus),
            neutral_damage_bonus:
                self.stats.as_base_characteristic(Type::NeutralDamageBonus),
            earth_damage_bonus:
                self.stats.as_base_characteristic(Type::EarthDamageBonus),
            water_damage_bonus:
                self.stats.as_base_characteristic(Type::WaterDamageBonus),
            air_damage_bonus:
                self.stats.as_base_characteristic(Type::AirDamageBonus),
            fire_damage_bonus:
                self.stats.as_base_characteristic(Type::FireDamageBonus),
            dodge_pa_lost_probability:
                self.stats.as_base_characteristic(Type::DodgePALostProbability),
            dodge_pm_lost_probability:
                self.stats.as_base_characteristic(Type::DodgePMLostProbability),
            neutral_element_resist_percent:
                self.stats.as_base_characteristic(Type::NeutralElementResistPercent),
            earth_element_resist_percent:
                self.stats.as_base_characteristic(Type::EarthElementResistPercent),
            water_element_resist_percent:
                self.stats.as_base_characteristic(Type::WaterElementResistPercent),
            air_element_resist_percent:
                self.stats.as_base_characteristic(Type::AirElementResistPercent),
            fire_element_resist_percent:
                self.stats.as_base_characteristic(Type::FireElementResistPercent),
            neutral_element_reduction:
                self.stats.as_base_characteristic(Type::NeutralElementReduction),
            earth_element_reduction:
                self.stats.as_base_characteristic(Type::EarthElementReduction),
            water_element_reduction:
                self.stats.as_base_characteristic(Type::WaterElementReduction),
            air_element_reduction:
                self.stats.as_base_characteristic(Type::AirElementReduction),
            fire_element_reduction:
                self.stats.as_base_characteristic(Type::FireElementReduction),
            push_damage_reduction:
                self.stats.as_base_characteristic(Type::PushDamageReduction),
            critical_damage_reduction:
                self.stats.as_base_characteristic(Type::CriticalDamageReduction),
            pvp_neutral_element_resist_percent:
                self.stats.as_base_characteristic(Type::PvpNeutralElementResistPercent),
            pvp_earth_element_resist_percent:
                self.stats.as_base_characteristic(Type::PvpEarthElementResistPercent),
            pvp_water_element_resist_percent:
                self.stats.as_base_characteristic(Type::PvpWaterElementResistPercent),
            pvp_air_element_resist_percent:
                self.stats.as_base_characteristic(Type::PvpAirElementResistPercent),
            pvp_fire_element_resist_percent:
                self.stats.as_base_characteristic(Type::PvpFireElementResistPercent),
            pvp_neutral_element_reduction:
                self.stats.as_base_characteristic(Type::PvpNeutralElementReduction),
            pvp_earth_element_reduction:
                self.stats.as_base_characteristic(Type::PvpEarthElementReduction),
            pvp_water_element_reduction:
                self.stats.as_base_characteristic(Type::PvpWaterElementReduction),
            pvp_air_element_reduction:
                self.stats.as_base_characteristic(Type::PvpAirElementReduction),
            pvp_fire_element_reduction:
                self.stats.as_base_characteristic(Type::PvpFireElementReduction),
            spell_modifications: Vec::new(),
            probation_time: 0,
        }
    }
}
