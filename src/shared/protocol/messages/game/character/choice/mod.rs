use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::types::game::character::choice::CharacterToRecolorInformation; use protocol::variants::CharacterBaseInformationsVariant; use protocol::messages::game::character::replay::CharacterReplayRequestMessage; use protocol::types::game::character::choice::CharacterToRemodelInformations; use protocol::types::game::character::choice::RemodelingInformation; use protocol::types::game::character::choice::CharacterToRelookInformation; use protocol::types::game::character::choice::CharacterBaseInformations;
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