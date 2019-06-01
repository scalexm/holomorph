pub mod arena;
pub mod breach;
pub mod challenge;
pub mod character;

use crate::types::game::action::fight::FightDispellableEffectExtendedInformations;
use crate::types::game::actions::fight::GameActionMark;
use crate::types::game::character::characteristic::CharacterCharacteristicsInformations;
use crate::types::game::context::fight::FightTeamInformations;
use crate::types::game::context::fight::GameFightMinimalStats;
use crate::types::game::context::fight::GameFightResumeSlaveInfo;
use crate::types::game::context::fight::GameFightSpellCooldown;
use crate::types::game::context::roleplay::party::NamedPartyTeam;
use crate::types::game::context::roleplay::party::NamedPartyTeamWithOutcome;
use crate::types::game::data::items::SpellItem;
use crate::types::game::idol::Idol;
use crate::variants::FightResultListEntryVariant;
use crate::variants::GameFightFighterInformationsVariant;
use crate::variants::ShortcutVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 715)]
pub struct GameFightTurnReadyRequestMessage<'a> {
    pub id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6543)]
pub struct GameFightPlacementSwapPositionsCancelMessage<'a> {
    pub request_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 740)]
pub struct GameFightHumanReadyStateMessage<'a> {
    #[protocol(var)]
    pub character_id: u64,
    pub is_ready: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6239)]
pub struct GameFightNewRoundMessage<'a> {
    #[protocol(var)]
    pub round_number: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 713)]
pub struct GameFightTurnListMessage<'a> {
    pub ids: std::borrow::Cow<'a, [f64]>,
    pub deads_ids: std::borrow::Cow<'a, [f64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 701)]
pub struct GameFightJoinRequestMessage<'a> {
    pub fighter_id: f64,
    #[protocol(var)]
    pub fight_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6754)]
pub struct GameFightPauseMessage<'a> {
    pub is_paused: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6474)]
pub struct GameFightSpectatePlayerRequestMessage<'a> {
    #[protocol(var)]
    pub player_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6547)]
pub struct GameFightPlacementSwapPositionsAcceptMessage<'a> {
    pub request_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 721)]
pub struct GameFightLeaveMessage<'a> {
    pub char_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 703)]
pub struct GameFightPlacementPossiblePositionsMessage<'a> {
    #[protocol(var_contents)]
    pub positions_for_challengers: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub positions_for_defenders: std::borrow::Cow<'a, [u16]>,
    pub team_number: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6548)]
pub struct GameFightPlacementSwapPositionsErrorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 707)]
pub struct GameFightOptionToggleMessage<'a> {
    pub option: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6541)]
pub struct GameFightPlacementSwapPositionsRequestMessage<'a> {
    pub base: GameFightPlacementPositionRequestMessage<'a>,
    pub requested_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6067)]
pub struct GameFightResumeMessage<'a> {
    pub base: GameFightSpectateMessage<'a>,
    pub spell_cooldowns: std::borrow::Cow<'a, [GameFightSpellCooldown<'a>]>,
    pub summon_count: u8,
    pub bomb_count: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 718)]
pub struct GameFightTurnFinishMessage<'a> {
    pub is_afk: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6544)]
pub struct GameFightPlacementSwapPositionsMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 711)]
pub struct GameFightRemoveTeamMemberMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub team_id: u8,
    pub char_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 702)]
pub struct GameFightJoinMessage<'a> {
    #[protocol(flag)]
    pub is_team_phase: bool,
    #[protocol(flag)]
    pub can_be_cancelled: bool,
    #[protocol(flag)]
    pub can_say_ready: bool,
    #[protocol(flag)]
    pub is_fight_started: bool,
    pub time_max_before_fight_start: u16,
    pub fight_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6465)]
pub struct GameFightTurnStartPlayingMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 719)]
pub struct GameFightTurnEndMessage<'a> {
    pub id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5921)]
pub struct GameFightSynchronizeMessage<'a> {
    pub fighters: std::borrow::Cow<'a, [GameFightFighterInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6214)]
pub struct SlaveSwitchContextMessage<'a> {
    pub master_id: f64,
    pub slave_id: f64,
    pub slave_spells: std::borrow::Cow<'a, [SpellItem<'a>]>,
    pub slave_stats: CharacterCharacteristicsInformations<'a>,
    pub shortcuts: std::borrow::Cow<'a, [ShortcutVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5927)]
pub struct GameFightOptionStateUpdateMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub team_id: u8,
    pub option: u8,
    pub state: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 716)]
pub struct GameFightTurnReadyMessage<'a> {
    pub is_ready: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 712)]
pub struct GameFightStartMessage<'a> {
    pub idols: std::borrow::Cow<'a, [Idol<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6069)]
pub struct GameFightSpectateMessage<'a> {
    pub effects: std::borrow::Cow<'a, [FightDispellableEffectExtendedInformations<'a>]>,
    pub marks: std::borrow::Cow<'a, [GameActionMark<'a>]>,
    #[protocol(var)]
    pub game_turn: u16,
    pub fight_start: u32,
    pub idols: std::borrow::Cow<'a, [Idol<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6307)]
pub struct GameFightTurnResumeMessage<'a> {
    pub base: GameFightTurnStartMessage<'a>,
    #[protocol(var)]
    pub remaining_time: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 714)]
pub struct GameFightTurnStartMessage<'a> {
    pub id: f64,
    #[protocol(var)]
    pub wait_time: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 708)]
pub struct GameFightReadyMessage<'a> {
    pub is_ready: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6215)]
pub struct GameFightResumeWithSlavesMessage<'a> {
    pub base: GameFightResumeMessage<'a>,
    pub slaves_info: std::borrow::Cow<'a, [GameFightResumeSlaveInfo<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6699)]
pub struct RefreshCharacterStatsMessage<'a> {
    pub fighter_id: f64,
    pub stats: GameFightMinimalStats<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6504)]
pub struct GameFightSpectatorJoinMessage<'a> {
    pub base: GameFightJoinMessage<'a>,
    pub named_party_teams: std::borrow::Cow<'a, [NamedPartyTeam<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 700)]
pub struct GameFightStartingMessage<'a> {
    pub fight_type: u8,
    #[protocol(var)]
    pub fight_id: u16,
    pub attacker_id: f64,
    pub defender_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 720)]
pub struct GameFightEndMessage<'a> {
    pub duration: u32,
    #[protocol(var)]
    pub reward_rate: i16,
    pub loot_share_limit_malus: i16,
    pub results: std::borrow::Cow<'a, [FightResultListEntryVariant<'a>]>,
    pub named_party_teams_outcomes: std::borrow::Cow<'a, [NamedPartyTeamWithOutcome<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6542)]
pub struct GameFightPlacementSwapPositionsOfferMessage<'a> {
    pub request_id: u32,
    pub requester_id: f64,
    #[protocol(var)]
    pub requester_cell_id: u16,
    pub requested_id: f64,
    #[protocol(var)]
    pub requested_cell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6490)]
pub struct GameFightNewWaveMessage<'a> {
    pub id: u8,
    pub team_id: u8,
    pub nb_turn_before_next_wave: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6546)]
pub struct GameFightPlacementSwapPositionsCancelledMessage<'a> {
    pub request_id: u32,
    pub canceller_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6807)]
pub struct SlaveNoLongerControledMessage<'a> {
    pub master_id: f64,
    pub slave_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 704)]
pub struct GameFightPlacementPositionRequestMessage<'a> {
    #[protocol(var)]
    pub cell_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5572)]
pub struct GameFightUpdateTeamMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub team: FightTeamInformations<'a>,
}
