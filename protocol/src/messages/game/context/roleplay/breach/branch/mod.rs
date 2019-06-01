use crate::types::game::context::roleplay::breach::ExtendedBreachBranch;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6812)]
pub struct BreachBranchesMessage<'a> {
    pub branches: std::borrow::Cow<'a, [ExtendedBreachBranch<'a>]>,
}
