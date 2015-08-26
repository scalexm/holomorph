use shared::protocol::types::game::look::EntityLook;
use shared::protocol::*;
use shared::protocol::types::game::character::*;
use shared::protocol::types::game::character::choice::*;
use shared::protocol::variants::CharacterBaseInformationsVariant;
use std::io::Cursor;
use postgres::rows::Row;

#[derive(Clone)]
pub struct CharacterMinimal {
    id: i32,
    account_id: i32,
    account_nickname: String,
    xp: i64,
    name: String,
    breed: i8,
    sex: bool,
    look: EntityLook,
}

impl CharacterMinimal {
    pub fn from_sql<'a>(row: Row<'a>) -> (i32, CharacterMinimal) {
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
            xp: row.get("xp"),
            name: row.get("name"),
            breed: breed as i8,
            sex: row.get("sex"),
            look: look,
        })
    }

    pub fn account_id(&self) -> i32 {
        self.account_id
    }

    pub fn as_character_base(&self) -> CharacterBaseInformationsVariant {
        CharacterBaseInformationsVariant::CharacterBaseInformations(
            CharacterBaseInformations {
                base: CharacterMinimalPlusLookInformations {
                    base: CharacterMinimalInformations {
                        base: AbstractCharacterInformation {
                            id: VarInt(self.id),
                        },
                        level: 200,
                        name: self.name.clone(),
                    },
                    entity_look: self.look.clone(),
                },
                breed: self.breed,
                sex: self.sex,
            }
        )
    }
}
