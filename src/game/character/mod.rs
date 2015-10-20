mod social;

use protocol::types::game::look::EntityLook;
use protocol::*;
use protocol::types::game::character::*;
use protocol::types::game::character::choice::*;
use protocol::types::game::character::characteristic::*;
use protocol::types::game::character::alignment::*;
use protocol::types::game::character::restriction::*;
use protocol::types::game::context::roleplay::*;
use protocol::types::game::context::*;
use std::io::Cursor;
use postgres::rows::Row;
use postgres::{self, Transaction};
use stats::{self, Type};
use stats::row::Field;
use shared::net::Token;
use session::game::SocialInformations;

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
    mood_smiley: i16,
    social: Option<SocialInformations>,
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
            mood_smiley: row.get("mood_smiley"),
            social: None,
        })
    }

    pub fn save<'a>(&self, trans: &Transaction<'a>) -> postgres::Result<()> {
        let mut look = Vec::new();
        self.look.serialize(&mut look).unwrap();

        let stmt = try!(trans.prepare_cached("UPDATE character_minimals SET level = $1,
            name = $2, breed = $3, sex = $4, look = $5, mood_smiley = $6 WHERE id = $7"));
        let _ = try!(stmt.execute(&[&self.level,
            &self.name,
            &(self.breed as i16),
            &self.sex,
            &look,
            &self.mood_smiley,
            &self.id]));

        Ok(())
    }

    pub fn set_mood_smiley(&mut self, mood: i16) {
        self.mood_smiley = mood;
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn account_id(&self) -> i32 {
        self.account_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn account_nickname(&self) -> &str {
        &self.account_nickname
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
    session: Token,
    xp: i64,
    kamas: i32,
    stats_points: i16,
    additionnal_points: i16,
    spells_points: i16,
    energy_points: i16,
    stats: stats::List,
    cell_id: i16,
    direction: i8,
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
        f.write_fmt(format_args!("invalid cell: {}", self.0))
    }
}

impl Character {
    pub fn from_sql<'a>(session: Token, base: CharacterMinimal, row: Row<'a>)
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

        let direction: i16 = try!(row.get_opt("direction"));
        Ok(Character {
            base: base,
            session: session,
            xp: try!(row.get_opt("xp")),
            kamas: try!(row.get_opt("kamas")),
            stats_points: try!(row.get_opt("stats_points")),
            additionnal_points: try!(row.get_opt("additionnal_points")),
            spells_points: try!(row.get_opt("spells_points")),
            energy_points: try!(row.get_opt("energy_points")),
            stats: stats,
            cell_id: cell_id,
            direction: direction as i8,
        })
    }

    pub fn save<'a>(&self, trans: &Transaction<'a>, map: i32) -> postgres::Result<()> {
        let stmt = try!(trans.prepare_cached("UPDATE characters SET base_vitality = $1,
            base_wisdom = $2, base_strength = $3, base_intelligence = $4, base_chance = $5,
            base_agility = $6, additionnal_vitality = $7, additionnal_wisdom = $8,
            additionnal_strength = $9, additionnal_intelligence = $10,
            additionnal_chance = $11, additionnal_agility = $12, cell_id = $13,
            map_id = $14, direction = $15, xp = $16, kamas = $17, stats_points = $18,
            additionnal_points = $19, spells_points = $20, energy_points = $21
            WHERE id = $22"));
        let _ = try!(stmt.execute(&[&self.stats.get(Type::Vitality, Field::Base),
                                    &self.stats.get(Type::Wisdom, Field::Base),
                                    &self.stats.get(Type::Strength, Field::Base),
                                    &self.stats.get(Type::Intelligence, Field::Base),
                                    &self.stats.get(Type::Chance, Field::Base),
                                    &self.stats.get(Type::Agility, Field::Base),
                                    &self.stats.get(Type::Vitality, Field::Additionnal),
                                    &self.stats.get(Type::Wisdom, Field::Additionnal),
                                    &self.stats.get(Type::Strength, Field::Additionnal),
                                    &self.stats.get(Type::Intelligence, Field::Additionnal),
                                    &self.stats.get(Type::Chance, Field::Additionnal),
                                    &self.stats.get(Type::Agility, Field::Additionnal),
                                    &self.cell_id,
                                    &map,
                                    &(self.direction as i16),
                                    &self.xp,
                                    &self.kamas,
                                    &self.stats_points,
                                    &self.additionnal_points,
                                    &self.spells_points,
                                    &self.energy_points,
                                    &self.base.id]));

        self.base.save(trans)
    }

    pub fn session(&self) -> Token {
        self.session
    }

    pub fn cell_id(&self) -> i16 {
        self.cell_id
    }

    pub fn set_cell_id(&mut self, id: i16) {
        self.cell_id = id;
    }

    pub fn set_direction(&mut self, dir: i8) {
        self.direction = dir;
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
                                direction: self.direction,
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
