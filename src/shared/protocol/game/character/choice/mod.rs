use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(BasicCharactersListMessage, 6475, characters| Vec<CharacterBaseInformationsVariant>);
impl_type!(CharacterFirstSelectionMessage, 6084, base| CharacterSelectionMessage, do_tutorial| bool);
impl_type!(CharacterReplayWithRemodelRequestMessage, 6551, base| CharacterReplayRequestMessage, remodel| RemodelingInformation);
impl_type!(CharacterSelectedErrorMessage, 5836);
impl_type!(CharacterSelectedForceMessage, 6068, id| i32);
impl_type!(CharacterSelectedForceReadyMessage, 6072);
impl_type!(CharacterSelectedSuccessMessage, 153, infos| CharacterBaseInformations, is_collecting_stats| bool);
impl_type!(CharacterSelectionMessage, 152, id| i32);
impl_type!(CharacterSelectionWithRemodelMessage, 6549, base| CharacterSelectionMessage, remodel| RemodelingInformation);
impl_type!(CharactersListErrorMessage, 5545);
impl_type!(CharactersListMessage, 151, base| BasicCharactersListMessage, has_startup_actions| bool);
impl_type!(CharactersListRequestMessage, 150);
impl_type!(CharactersListWithModificationsMessage, 6120, base| CharactersListMessage, characters_to_recolor| Vec<CharacterToRecolorInformation>, characters_to_rename| Vec<i32>, unusable_characters| Vec<i32>, characters_to_relook| Vec<CharacterToRelookInformation>);
impl_type!(CharactersListWithRemodelingMessage, 6550, base| CharactersListMessage, characters_to_remodel| Vec<CharacterToRemodelInformations>);

impl_type!(AbstractCharacterToRefurbishInformation, 475, base| AbstractCharacterInformation, colors| Vec<i32>, cosmetic_id| VarInt);
impl_type!(CharacterBaseInformations, 45, base| CharacterMinimalPlusLookInformations, breed| i8, sex| bool);
impl_type!(CharacterHardcoreOrEpicInformations, 474, base| CharacterBaseInformations, death_state| i8, death_count| VarShort, death_max_level| i8);
impl_type!(CharacterRemodelingInformation, 479, base| AbstractCharacterInformation, name| String, breed| i8, sex| bool, cosmetic_id| VarShort, colors| Vec<i32>);
impl_type!(CharacterToRecolorInformation, 212, base| AbstractCharacterToRefurbishInformation);
impl_type!(CharacterToRelookInformation, 399, base| AbstractCharacterToRefurbishInformation);
impl_type!(CharacterToRemodelInformations, 477, base| CharacterRemodelingInformation, possible_change_mask| i8, mandatory_change_mask| i8);
impl_type!(RemodelingInformation, 480, name| String, breed| i8, sex| bool, cosmetic_id| VarShort, colors| Vec<i32>);
