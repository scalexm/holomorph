use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

use types::game::interactive::*;
use types::game::character::status::*;
use types::game::character::choice::*;
use types::game::approach::*;
use types::game::idol::*;
use types::game::shortcut::*;
use types::game::context::*;
use types::game::guild::tax::*;
use types::game::social::*;
use types::game::context::roleplay::*;
use types::game::context::fight::*;
use types::game::prism::*;
use types::game::character::*;
use types::game::context::roleplay::quest::*;
use types::game::interactive::skill::*;
use types::game::context::roleplay::treasure_hunt::*;
use types::game::house::*;
use types::game::friend::*;
use types::game::paddock::*;
use types::game::context::roleplay::party::*;
use types::game::mount::*;
use types::game::actions::fight::*;
use types::game::data::items::effects::*;

impl_variant!(
    InteractiveElementSkillVariant,
    InteractiveElementSkill| InteractiveElementSkill,
    InteractiveElementNamedSkill| InteractiveElementNamedSkill
);

impl_variant!(
    InteractiveElementVariant,
    InteractiveElement| InteractiveElement,
    InteractiveElementWithAgeBonus| InteractiveElementWithAgeBonus
);

impl_variant!(
    PlayerStatusVariant,
    PlayerStatus| PlayerStatus,
    PlayerStatusExtended| PlayerStatusExtended
);

impl PlayerStatusVariant {
    pub fn status_id(&self) -> i8 {
        match *self {
            PlayerStatusVariant::PlayerStatus(ref s) => s.status_id,
            PlayerStatusVariant::PlayerStatusExtended(ref s) => s.base.status_id,
        }
    }
}

impl_variant!(
    CharacterBaseInformationsVariant,
    CharacterBaseInformations| CharacterBaseInformations,
    CharacterHardcoreOrEpicInformations| CharacterHardcoreOrEpicInformations
);

impl_variant!(
    ServerSessionConstantVariant,
    ServerSessionConstant| ServerSessionConstant,
    ServerSessionConstantInteger| ServerSessionConstantInteger,
    ServerSessionConstantLong| ServerSessionConstantLong,
    ServerSessionConstantString| ServerSessionConstantString
);

impl_variant!(
    IdolVariant,
    Idol| Idol,
    PartyIdol| PartyIdol
);

impl_variant!(
    ShortcutVariant,
    Shortcut| Shortcut,
    ShortcutEmote| ShortcutEmote,
    ShortcutObject| ShortcutObject,
    ShortcutObjectItem| ShortcutObjectItem,
    ShortcutObjectPreset| ShortcutObjectPreset,
    ShortcutSmiley| ShortcutSmiley,
    ShortcutSpell| ShortcutSpell
);

impl_variant!(
    MapCoordinatesVariant,
    MapCoordinates| MapCoordinates,
    MapCoordinatesAndId| MapCoordinatesAndId,
    MapCoordinatesExtended| MapCoordinatesExtended
);

impl_variant!(
    TaxCollectorInformationsVariant,
    TaxCollectorInformations| TaxCollectorInformations
);

impl_variant!(
    GuildVersatileInformationsVariant,
    GuildVersatileInformations| GuildVersatileInformations,
    GuildInAllianceVersatileInformations| GuildInAllianceVersatileInformations
);

impl_variant!(
    GuildFactSheetInformationsVariant,
    GuildFactSheetInformations| GuildFactSheetInformations,
    GuildInsiderFactSheetInformations| GuildInsiderFactSheetInformations
);

impl_variant!(
    GameContextActorInformationsVariant,
    GameContextActorInformations| GameContextActorInformations,

    GameFightFighterInformations| GameFightFighterInformations,
    GameFightCompanionInformations| GameFightCompanionInformations,
    GameFightAIInformations| GameFightAIInformations,
    GameFightFighterNamedInformations| GameFightFighterNamedInformations,
    GameFightMonsterInformations| GameFightMonsterInformations,
    GameFightCharacterInformations| GameFightCharacterInformations,
    GameFightMonsterWithAlignmentInformations| GameFightMonsterWithAlignmentInformations,
    GameFightMutantInformations| GameFightMutantInformations,
    GameFightTaxCollectorInformations| GameFightTaxCollectorInformations,

    GameRolePlayTaxCollectorInformations| GameRolePlayTaxCollectorInformations,
    GameRolePlayActorInformations| GameRolePlayActorInformations,
    GameRolePlayNamedActorInformations| GameRolePlayNamedActorInformations,
    GameRolePlayNpcInformations| GameRolePlayNpcInformations,
    GameRolePlayNpcWithQuestInformations| GameRolePlayNpcWithQuestInformations,
    GameRolePlayPortalInformations| GameRolePlayPortalInformations,
    GameRolePlayPrismInformations| GameRolePlayPrismInformations,
    GameRolePlayTreasureHintInformations| GameRolePlayTreasureHintInformations,
    GameRolePlayMerchantInformations| GameRolePlayMerchantInformations,
    GameRolePlayHumanoidInformations| GameRolePlayHumanoidInformations,
    GameRolePlayMutantInformations| GameRolePlayMutantInformations,
    GameRolePlayCharacterInformations| GameRolePlayCharacterInformations,
    GameRolePlayGroupMonsterInformations| GameRolePlayGroupMonsterInformations,
    GameRolePlayGroupMonsterWaveInformations| GameRolePlayGroupMonsterWaveInformations
);

impl_variant!(
    PrismInformationVariant,
    PrismInformation| PrismInformation,
    AlliancePrismInformation| AlliancePrismInformation,
    AllianceInsiderPrismInformation| AllianceInsiderPrismInformation
);

impl_variant!(
    CharacterMinimalPlusLookInformationsVariant,
    CharacterMinimalPlusLookInformations| CharacterMinimalPlusLookInformations,
    CharacterMinimalPlusLookAndGradeInformations| CharacterMinimalPlusLookAndGradeInformations
);

impl_variant!(
    QuestObjectiveInformationsVariant,
    QuestObjectiveInformations| QuestObjectiveInformations,
    QuestObjectiveInformationsWithCompletion| QuestObjectiveInformationsWithCompletion
);

impl_variant!(
    SkillActionDescriptionVariant,
    SkillActionDescription| SkillActionDescription,
    SkillActionDescriptionCollect| SkillActionDescriptionCollect,
    SkillActionDescriptionCraft| SkillActionDescriptionCraft,
    SkillActionDescriptionTimed| SkillActionDescriptionTimed
);

impl_variant!(
    HumanOptionVariant,
    HumanOption| HumanOption,
    HumanOptionAlliance| HumanOptionAlliance,
    HumanOptionEmote| HumanOptionEmote,
    HumanOptionFollowers| HumanOptionFollowers,
    HumanOptionGuild| HumanOptionGuild,
    HumanOptionObjectUse| HumanOptionObjectUse,
    HumanOptionOrnament| HumanOptionOrnament,
    HumanOptionTitle| HumanOptionTitle
);

impl_variant!(
    GroupMonsterStaticInformationsVariant,
    GroupMonsterStaticInformations| GroupMonsterStaticInformations,
    GroupMonsterStaticInformationsWithAlternatives| GroupMonsterStaticInformationsWithAlternatives
);

impl_variant!(
    PortalInformationVariant,
    PortalInformation| PortalInformation
);

impl_variant!(
    HumanInformationsVariant,
    HumanInformations| HumanInformations
);

impl_variant!(
    FightTeamMemberInformationsVariant,
    FightTeamMemberInformations| FightTeamMemberInformations,
    FightTeamMemberCompanionInformations| FightTeamMemberCompanionInformations,
    FightTeamMemberCharacterInformations| FightTeamMemberCharacterInformations,
    FightTeamMemberMonsterInformations| FightTeamMemberMonsterInformations,
    FightTeamMemberTaxCollectorInformations| FightTeamMemberTaxCollectorInformations,
    FightTeamMemberWithAllianceCharacterInformations| FightTeamMemberWithAllianceCharacterInformations
);

impl_variant!(
    GameFightMinimalStatsVariant,
    GameFightMinimalStats| GameFightMinimalStats,
    GameFightMinimalStatsPreparation| GameFightMinimalStatsPreparation
);

impl_variant!(
    ObjectEffectVariant,
    ObjectEffect| ObjectEffect,
    ObjectEffectCreature| ObjectEffectCreature,
    ObjectEffectDate| ObjectEffectDate,
    ObjectEffectDice| ObjectEffectDice,
    ObjectEffectDuration| ObjectEffectDuration,
    ObjectEffectInteger| ObjectEffectInteger,
    ObjectEffectLadder| ObjectEffectLadder,
    ObjectEffectMinMax| ObjectEffectMinMax,
    ObjectEffectMount| ObjectEffectMount,
    ObjectEffectString| ObjectEffectString
);

impl_variant!(
    AbstractFightDispellableEffectVariant,
    AbstractFightDispellableEffect| AbstractFightDispellableEffect,
    FightTemporaryBoostEffect| FightTemporaryBoostEffect,
    FightTemporaryBoostStateEffect| FightTemporaryBoostStateEffect,
    FightTemporaryBoostWeaponDamagesEffect| FightTemporaryBoostWeaponDamagesEffect,
    FightTemporarySpellBoostEffect| FightTemporarySpellBoostEffect,
    FightTemporarySpellImmunityEffect| FightTemporarySpellImmunityEffect,
    FightTriggeredEffect| FightTriggeredEffect
);

impl_variant!(
    TaxCollectorComplementaryInformationsVariant,
    TaxCollectorComplementaryInformations| TaxCollectorComplementaryInformations,
    TaxCollectorGuildInformations| TaxCollectorGuildInformations,
    TaxCollectorLootInformations| TaxCollectorLootInformations,
    TaxCollectorWaitingForHelpInformations| TaxCollectorWaitingForHelpInformations
);

impl_variant!(
    FightTeamInformationsVariant,
    FightTeamInformations| FightTeamInformations,
    FightAllianceTeamInformations| FightAllianceTeamInformations
);

impl_variant!(
    FightResultAdditionalDataVariant,
    FightResultAdditionalData| FightResultAdditionalData,
    FightResultExperienceData| FightResultExperienceData,
    FightResultPvpData| FightResultPvpData
);

impl_variant!(
    EntityDispositionInformationsVariant,
    EntityDispositionInformations| EntityDispositionInformations,
    FightEntityDispositionInformations| FightEntityDispositionInformations,
    IdentifiedEntityDispositionInformations| IdentifiedEntityDispositionInformations
);

impl_variant!(
    AllianceFactSheetInformationsVariant,
    AllianceFactSheetInformations| AllianceFactSheetInformations
);

impl_variant!(
    PrismSubareaEmptyInfoVariant,
    PrismSubareaEmptyInfo| PrismSubareaEmptyInfo,
    PrismGeolocalizedInformation| PrismGeolocalizedInformation
);

impl_variant!(
    UpdateMountBoostVariant,
    UpdateMountBoost| UpdateMountBoost,
    UpdateMountIntBoost| UpdateMountIntBoost
);

impl_variant!(
    HouseInformationsVariant,
    HouseInformations| HouseInformations,
    HouseInformationsExtended| HouseInformationsExtended
);

impl_variant!(
    AbstractSocialGroupInfosVariant,
    AbstractSocialGroupInfos| AbstractSocialGroupInfos,
    BasicGuildInformations| BasicGuildInformations,
    BasicAllianceInformations| BasicAllianceInformations
);

impl_variant!(
    GameFightFighterInformationsVariant,
    GameFightFighterInformations| GameFightFighterInformations,
    GameFightCompanionInformations| GameFightCompanionInformations,
    GameFightAIInformations| GameFightAIInformations,
    GameFightFighterNamedInformations| GameFightFighterNamedInformations,
    GameFightMonsterInformations| GameFightMonsterInformations,
    GameFightCharacterInformations| GameFightCharacterInformations,
    GameFightMonsterWithAlignmentInformations| GameFightMonsterWithAlignmentInformations,
    GameFightMutantInformations| GameFightMutantInformations,
    GameFightTaxCollectorInformations| GameFightTaxCollectorInformations
);

impl_variant!(
    FriendInformationsVariant,
    FriendInformations| FriendInformations,
    FriendOnlineInformations| FriendOnlineInformations
);

impl FriendInformationsVariant {
    pub fn name(&self) -> &str {
        match *self {
            FriendInformationsVariant::FriendInformations(ref infos) => &infos.base.account_name,
            FriendInformationsVariant::FriendOnlineInformations(ref infos) => {
                &infos.base.base.account_name
            },
        }
    }
}

impl_type!(SessionIgnoredInformations, 9999, name| String);

impl_variant!(
    IgnoredInformationsVariant,
    IgnoredInformations| IgnoredInformations,
    IgnoredOnlineInformations| IgnoredOnlineInformations,
    SessionIgnoredInformations| SessionIgnoredInformations
);

impl IgnoredInformationsVariant {
    pub fn name(&self) -> &str {
        match *self {
            IgnoredInformationsVariant::IgnoredInformations(ref infos) => &infos.base.account_name,
            IgnoredInformationsVariant::IgnoredOnlineInformations(ref infos) => {
                &infos.base.base.account_name
            },
            IgnoredInformationsVariant::SessionIgnoredInformations(ref infos) => &infos.name,
        }
    }
}

impl_variant!(
    FriendSpouseInformationsVariant,
    FriendSpouseInformations| FriendSpouseInformations,
    FriendSpouseOnlineInformations| FriendSpouseOnlineInformations
);

impl_variant!(
    PaddockInformationsVariant,
    PaddockInformations| PaddockInformations,
    PaddockContentInformations| PaddockContentInformations,
    PaddockBuyableInformations| PaddockBuyableInformations,
    PaddockAbandonnedInformations| PaddockAbandonnedInformations,
    PaddockPrivateInformations| PaddockPrivateInformations
);

impl_variant!(
    FightResultListEntryVariant,
    FightResultListEntry| FightResultListEntry,
    FightResultFighterListEntry| FightResultFighterListEntry,
    FightResultMutantListEntry| FightResultMutantListEntry,
    FightResultPlayerListEntry| FightResultPlayerListEntry,
    FightResultTaxCollectorListEntry| FightResultTaxCollectorListEntry
);

impl_variant!(
    TreasureHuntStepVariant,
    TreasureHuntStep| TreasureHuntStep,
    TreasureHuntStepDig| TreasureHuntStepDig,
    TreasureHuntStepFight| TreasureHuntStepFight,
    TreasureHuntStepFollowDirection| TreasureHuntStepFollowDirection,
    TreasureHuntStepFollowDirectionToHint| TreasureHuntStepFollowDirectionToHint,
    TreasureHuntStepFollowDirectionToPOI| TreasureHuntStepFollowDirectionToPOI
);

impl_variant!(
    PartyMemberInformationsVariant,
    PartyMemberInformations| PartyMemberInformations,
    PartyMemberArenaInformations| PartyMemberArenaInformations
);

impl_variant!(
    QuestActiveInformationsVariant,
    QuestActiveInformations| QuestActiveInformations,
    QuestActiveDetailedInformations| QuestActiveDetailedInformations
);

impl_variant!(
    TaxCollectorStaticInformationsVariant,
    TaxCollectorStaticInformations| TaxCollectorStaticInformations,
    TaxCollectorStaticExtendedInformations| TaxCollectorStaticExtendedInformations
);

impl_variant!(
    PartyIdolVariant,
    PartyIdol| PartyIdol
);

impl_variant!(
    GameRolePlayActorInformationsVariant,
    GameRolePlayTaxCollectorInformations| GameRolePlayTaxCollectorInformations,
    GameRolePlayActorInformations| GameRolePlayActorInformations,
    GameRolePlayNamedActorInformations| GameRolePlayNamedActorInformations,
    GameRolePlayNpcInformations| GameRolePlayNpcInformations,
    GameRolePlayNpcWithQuestInformations| GameRolePlayNpcWithQuestInformations,
    GameRolePlayPortalInformations| GameRolePlayPortalInformations,
    GameRolePlayPrismInformations| GameRolePlayPrismInformations,
    GameRolePlayTreasureHintInformations| GameRolePlayTreasureHintInformations,
    GameRolePlayMerchantInformations| GameRolePlayMerchantInformations,
    GameRolePlayHumanoidInformations| GameRolePlayHumanoidInformations,
    GameRolePlayMutantInformations| GameRolePlayMutantInformations,
    GameRolePlayCharacterInformations| GameRolePlayCharacterInformations,
    GameRolePlayGroupMonsterInformations| GameRolePlayGroupMonsterInformations,
    GameRolePlayGroupMonsterWaveInformations| GameRolePlayGroupMonsterWaveInformations
);

impl_variant!(
    GameFightFighterLightInformationsVariant,
    GameFightFighterLightInformations| GameFightFighterLightInformations,
    GameFightFighterNamedLightInformations| GameFightFighterNamedLightInformations,
    GameFightFighterMonsterLightInformations| GameFightFighterMonsterLightInformations,
    GameFightFighterTaxCollectorLightInformations| GameFightFighterTaxCollectorLightInformations,
    GameFightFighterCompanionLightInformations| GameFightFighterCompanionLightInformations
);
