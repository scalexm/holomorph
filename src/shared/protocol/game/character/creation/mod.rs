use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(CharacterCreationRequestMessage, 160, name| String, breed| i8, sex| bool, colors| Vec<i32>, cosmetic_id| VarShort);
impl_type!(CharacterCreationResultMessage, 161, result| i8);
impl_type!(CharacterNameSuggestionFailureMessage, 164, reason| i8);
impl_type!(CharacterNameSuggestionRequestMessage, 162);
impl_type!(CharacterNameSuggestionSuccessMessage, 5544, suggestion| String);
