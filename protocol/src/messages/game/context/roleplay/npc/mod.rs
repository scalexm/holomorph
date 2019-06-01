use crate::types::game::context::roleplay::quest::GameRolePlayNpcQuestFlag;
use crate::types::game::context::roleplay::BasicGuildInformations;
use crate::types::game::context::roleplay::BasicNamedAllianceInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6110)]
pub struct EntityTalkMessage<'a> {
    pub entity_id: f64,
    #[protocol(var)]
    pub text_id: u16,
    pub parameters: std::borrow::Cow<'a, [&'a str]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5618)]
pub struct NpcDialogCreationMessage<'a> {
    pub map_id: f64,
    pub npc_id: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6737)]
pub struct PortalDialogCreationMessage<'a> {
    pub base: NpcDialogCreationMessage<'a>,
    pub type_: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5898)]
pub struct NpcGenericActionRequestMessage<'a> {
    pub npc_id: i32,
    pub npc_action_id: u8,
    pub npc_map_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5642)]
pub struct MapNpcsQuestStatusUpdateMessage<'a> {
    pub map_id: f64,
    pub npcs_ids_with_quest: std::borrow::Cow<'a, [i32]>,
    pub quest_flags: std::borrow::Cow<'a, [GameRolePlayNpcQuestFlag<'a>]>,
    pub npcs_ids_without_quest: std::borrow::Cow<'a, [i32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5900)]
pub struct NpcGenericActionFailureMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5615)]
pub struct TaxCollectorDialogQuestionExtendedMessage<'a> {
    pub base: TaxCollectorDialogQuestionBasicMessage<'a>,
    #[protocol(var)]
    pub max_pods: u16,
    #[protocol(var)]
    pub prospecting: u16,
    #[protocol(var)]
    pub wisdom: u16,
    pub tax_collectors_count: u8,
    pub tax_collector_attack: i32,
    #[protocol(var)]
    pub kamas: u64,
    #[protocol(var)]
    pub experience: u64,
    #[protocol(var)]
    pub pods: u32,
    #[protocol(var)]
    pub items_value: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5617)]
pub struct NpcDialogQuestionMessage<'a> {
    #[protocol(var)]
    pub message_id: u32,
    pub dialog_params: std::borrow::Cow<'a, [&'a str]>,
    #[protocol(var_contents)]
    pub visible_replies: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5616)]
pub struct NpcDialogReplyMessage<'a> {
    #[protocol(var)]
    pub reply_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5619)]
pub struct TaxCollectorDialogQuestionBasicMessage<'a> {
    pub guild_info: BasicGuildInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6448)]
pub struct AlliancePrismDialogQuestionMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6445)]
pub struct AllianceTaxCollectorDialogQuestionExtendedMessage<'a> {
    pub base: TaxCollectorDialogQuestionExtendedMessage<'a>,
    pub alliance: BasicNamedAllianceInformations<'a>,
}
