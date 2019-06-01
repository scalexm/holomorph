use protocol_derive::{Decode, Encode};

use crate::types::common::basic::*;
use crate::types::game::actions::fight::*;
use crate::types::game::approach::*;
use crate::types::game::character::choice::*;
use crate::types::game::character::status::*;
use crate::types::game::character::*;
use crate::types::game::context::fight::*;
use crate::types::game::context::roleplay::party::*;
use crate::types::game::context::roleplay::quest::*;
use crate::types::game::context::roleplay::treasure_hunt::*;
use crate::types::game::context::roleplay::*;
use crate::types::game::context::*;
use crate::types::game::data::items::effects::*;
use crate::types::game::friend::*;
use crate::types::game::guild::tax::*;
use crate::types::game::idol::*;
use crate::types::game::interactive::skill::*;
use crate::types::game::interactive::*;
use crate::types::game::paddock::*;
use crate::types::game::prism::*;
use crate::types::game::shortcut::*;
use crate::types::game::social::*;

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum InteractiveElementSkillVariant<'a> {
    InteractiveElementSkill(InteractiveElementSkill<'a>),
    InteractiveElementNamedSkill(InteractiveElementNamedSkill<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum InteractiveElementVariant<'a> {
    InteractiveElement(InteractiveElement<'a>),
    InteractiveElementWithAgeBonus(InteractiveElementWithAgeBonus<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PlayerStatusVariant<'a> {
    PlayerStatus(PlayerStatus<'a>),
    PlayerStatusExtended(PlayerStatusExtended<'a>),
}

impl PlayerStatusVariant<'_> {
    pub fn status_id(&self) -> u8 {
        match self {
            PlayerStatusVariant::PlayerStatus(s) => s.status_id,
            PlayerStatusVariant::PlayerStatusExtended(s) => s.base.status_id,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum CharacterBaseInformationsVariant<'a> {
    CharacterBaseInformations(CharacterBaseInformations<'a>),
    CharacterHardcoreOrEpicInformations(CharacterHardcoreOrEpicInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum ServerSessionConstantVariant<'a> {
    ServerSessionConstant(ServerSessionConstant<'a>),
    ServerSessionConstantInteger(ServerSessionConstantInteger<'a>),
    ServerSessionConstantLong(ServerSessionConstantLong<'a>),
    ServerSessionConstantString(ServerSessionConstantString<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum IdolVariant<'a> {
    Idol(Idol<'a>),
    PartyIdol(PartyIdol<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum ShortcutVariant<'a> {
    Shortcut(Shortcut<'a>),
    ShortcutEmote(ShortcutEmote<'a>),
    ShortcutObject(ShortcutObject<'a>),
    ShortcutObjectItem(ShortcutObjectItem<'a>),
    ShortcutObjectPreset(ShortcutObjectPreset<'a>),
    ShortcutSmiley(ShortcutSmiley<'a>),
    ShortcutSpell(ShortcutSpell<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum MapCoordinatesVariant<'a> {
    MapCoordinates(MapCoordinates<'a>),
    MapCoordinatesAndId(MapCoordinatesAndId<'a>),
    MapCoordinatesExtended(MapCoordinatesExtended<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum TaxCollectorInformationsVariant<'a> {
    TaxCollectorInformations(TaxCollectorInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum GuildVersatileInformationsVariant<'a> {
    GuildVersatileInformations(GuildVersatileInformations<'a>),
    GuildInAllianceVersatileInformations(GuildInAllianceVersatileInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum GuildFactSheetInformationsVariant<'a> {
    GuildFactSheetInformations(GuildFactSheetInformations<'a>),
    GuildInsiderFactSheetInformations(GuildInsiderFactSheetInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum GameContextActorInformationsVariant<'a> {
    GameContextActorInformations(GameContextActorInformations<'a>),

    GameFightFighterInformations(GameFightFighterInformations<'a>),
    GameFightAIInformations(GameFightAIInformations<'a>),
    GameFightFighterNamedInformations(GameFightFighterNamedInformations<'a>),
    GameFightMonsterInformations(GameFightMonsterInformations<'a>),
    GameFightCharacterInformations(GameFightCharacterInformations<'a>),
    GameFightMonsterWithAlignmentInformations(GameFightMonsterWithAlignmentInformations<'a>),
    GameFightMutantInformations(GameFightMutantInformations<'a>),
    GameFightTaxCollectorInformations(GameFightTaxCollectorInformations<'a>),

    GameRolePlayTaxCollectorInformations(GameRolePlayTaxCollectorInformations<'a>),
    GameRolePlayActorInformations(GameRolePlayActorInformations<'a>),
    GameRolePlayNamedActorInformations(GameRolePlayNamedActorInformations<'a>),
    GameRolePlayNpcInformations(GameRolePlayNpcInformations<'a>),
    GameRolePlayNpcWithQuestInformations(GameRolePlayNpcWithQuestInformations<'a>),
    GameRolePlayPortalInformations(GameRolePlayPortalInformations<'a>),
    GameRolePlayPrismInformations(GameRolePlayPrismInformations<'a>),
    GameRolePlayTreasureHintInformations(GameRolePlayTreasureHintInformations<'a>),
    GameRolePlayMerchantInformations(GameRolePlayMerchantInformations<'a>),
    GameRolePlayHumanoidInformations(GameRolePlayHumanoidInformations<'a>),
    GameRolePlayMutantInformations(GameRolePlayMutantInformations<'a>),
    GameRolePlayCharacterInformations(GameRolePlayCharacterInformations<'a>),
    GameRolePlayGroupMonsterInformations(GameRolePlayGroupMonsterInformations<'a>),
    GameRolePlayGroupMonsterWaveInformations(GameRolePlayGroupMonsterWaveInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PrismInformationVariant<'a> {
    PrismInformation(PrismInformation<'a>),
    AlliancePrismInformation(AlliancePrismInformation<'a>),
    AllianceInsiderPrismInformation(AllianceInsiderPrismInformation<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum CharacterMinimalPlusLookInformationsVariant<'a> {
    CharacterMinimalPlusLookInformations(CharacterMinimalPlusLookInformations<'a>),
    CharacterMinimalPlusLookAndGradeInformations(CharacterMinimalPlusLookAndGradeInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum QuestObjectiveInformationsVariant<'a> {
    QuestObjectiveInformations(QuestObjectiveInformations<'a>),
    QuestObjectiveInformationsWithCompletion(QuestObjectiveInformationsWithCompletion<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum SkillActionDescriptionVariant<'a> {
    SkillActionDescription(SkillActionDescription<'a>),
    SkillActionDescriptionCollect(SkillActionDescriptionCollect<'a>),
    SkillActionDescriptionCraft(SkillActionDescriptionCraft<'a>),
    SkillActionDescriptionTimed(SkillActionDescriptionTimed<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum HumanOptionVariant<'a> {
    HumanOption(HumanOption<'a>),
    HumanOptionAlliance(HumanOptionAlliance<'a>),
    HumanOptionEmote(HumanOptionEmote<'a>),
    HumanOptionFollowers(HumanOptionFollowers<'a>),
    HumanOptionGuild(HumanOptionGuild<'a>),
    HumanOptionObjectUse(HumanOptionObjectUse<'a>),
    HumanOptionOrnament(HumanOptionOrnament<'a>),
    HumanOptionTitle(HumanOptionTitle<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum GroupMonsterStaticInformationsVariant<'a> {
    GroupMonsterStaticInformations(GroupMonsterStaticInformations<'a>),
    GroupMonsterStaticInformationsWithAlternatives(
        GroupMonsterStaticInformationsWithAlternatives<'a>,
    ),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PortalInformationVariant<'a> {
    PortalInformation(PortalInformation<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum HumanInformationsVariant<'a> {
    HumanInformations(HumanInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum FightTeamMemberInformationsVariant<'a> {
    FightTeamMemberInformations(FightTeamMemberInformations<'a>),
    FightTeamMemberCharacterInformations(FightTeamMemberCharacterInformations<'a>),
    FightTeamMemberMonsterInformations(FightTeamMemberMonsterInformations<'a>),
    FightTeamMemberTaxCollectorInformations(FightTeamMemberTaxCollectorInformations<'a>),
    FightTeamMemberWithAllianceCharacterInformations(
        FightTeamMemberWithAllianceCharacterInformations<'a>,
    ),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum GameFightMinimalStatsVariant<'a> {
    GameFightMinimalStats(GameFightMinimalStats<'a>),
    GameFightMinimalStatsPreparation(GameFightMinimalStatsPreparation<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum ObjectEffectVariant<'a> {
    ObjectEffect(ObjectEffect<'a>),
    ObjectEffectCreature(ObjectEffectCreature<'a>),
    ObjectEffectDate(ObjectEffectDate<'a>),
    ObjectEffectDice(ObjectEffectDice<'a>),
    ObjectEffectDuration(ObjectEffectDuration<'a>),
    ObjectEffectInteger(ObjectEffectInteger<'a>),
    ObjectEffectLadder(ObjectEffectLadder<'a>),
    ObjectEffectMinMax(ObjectEffectMinMax<'a>),
    ObjectEffectMount(ObjectEffectMount<'a>),
    ObjectEffectString(ObjectEffectString<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum AbstractFightDispellableEffectVariant<'a> {
    AbstractFightDispellableEffect(AbstractFightDispellableEffect<'a>),
    FightTemporaryBoostEffect(FightTemporaryBoostEffect<'a>),
    FightTemporaryBoostStateEffect(FightTemporaryBoostStateEffect<'a>),
    FightTemporaryBoostWeaponDamagesEffect(FightTemporaryBoostWeaponDamagesEffect<'a>),
    FightTemporarySpellBoostEffect(FightTemporarySpellBoostEffect<'a>),
    FightTemporarySpellImmunityEffect(FightTemporarySpellImmunityEffect<'a>),
    FightTriggeredEffect(FightTriggeredEffect<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum TaxCollectorComplementaryInformationsVariant<'a> {
    TaxCollectorComplementaryInformations(TaxCollectorComplementaryInformations<'a>),
    TaxCollectorGuildInformations(TaxCollectorGuildInformations<'a>),
    TaxCollectorLootInformations(TaxCollectorLootInformations<'a>),
    TaxCollectorWaitingForHelpInformations(TaxCollectorWaitingForHelpInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum FightTeamInformationsVariant<'a> {
    FightTeamInformations(FightTeamInformations<'a>),
    FightAllianceTeamInformations(FightAllianceTeamInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum FightResultAdditionalDataVariant<'a> {
    FightResultAdditionalData(FightResultAdditionalData<'a>),
    FightResultExperienceData(FightResultExperienceData<'a>),
    FightResultPvpData(FightResultPvpData<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum EntityDispositionInformationsVariant<'a> {
    EntityDispositionInformations(EntityDispositionInformations<'a>),
    FightEntityDispositionInformations(FightEntityDispositionInformations<'a>),
    IdentifiedEntityDispositionInformations(IdentifiedEntityDispositionInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum AllianceFactSheetInformationsVariant<'a> {
    AllianceFactSheetInformations(AllianceFactSheetInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PrismSubareaEmptyInfoVariant<'a> {
    PrismSubareaEmptyInfo(PrismSubareaEmptyInfo<'a>),
    PrismGeolocalizedInformation(PrismGeolocalizedInformation<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum AbstractSocialGroupInfosVariant<'a> {
    AbstractSocialGroupInfos(AbstractSocialGroupInfos<'a>),
    BasicGuildInformations(BasicGuildInformations<'a>),
    BasicAllianceInformations(BasicAllianceInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum GameFightFighterInformationsVariant<'a> {
    GameFightFighterInformations(GameFightFighterInformations<'a>),
    GameFightAIInformations(GameFightAIInformations<'a>),
    GameFightFighterNamedInformations(GameFightFighterNamedInformations<'a>),
    GameFightMonsterInformations(GameFightMonsterInformations<'a>),
    GameFightCharacterInformations(GameFightCharacterInformations<'a>),
    GameFightMonsterWithAlignmentInformations(GameFightMonsterWithAlignmentInformations<'a>),
    GameFightMutantInformations(GameFightMutantInformations<'a>),
    GameFightTaxCollectorInformations(GameFightTaxCollectorInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum FriendInformationsVariant<'a> {
    FriendInformations(FriendInformations<'a>),
    FriendOnlineInformations(FriendOnlineInformations<'a>),
}

impl<'a> FriendInformationsVariant<'a> {
    pub fn name(&self) -> &'a str {
        match self {
            FriendInformationsVariant::FriendInformations(infos) => infos.base.account_name,
            FriendInformationsVariant::FriendOnlineInformations(infos) => {
                infos.base.base.account_name
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum IgnoredInformationsVariant<'a> {
    IgnoredInformations(IgnoredInformations<'a>),
    IgnoredOnlineInformations(IgnoredOnlineInformations<'a>),
}

impl<'a> IgnoredInformationsVariant<'a> {
    pub fn name(&self) -> &'a str {
        match self {
            IgnoredInformationsVariant::IgnoredInformations(infos) => infos.base.account_name,
            IgnoredInformationsVariant::IgnoredOnlineInformations(infos) => {
                infos.base.base.account_name
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum FriendSpouseInformationsVariant<'a> {
    FriendSpouseInformations(FriendSpouseInformations<'a>),
    FriendSpouseOnlineInformations(FriendSpouseOnlineInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PaddockInformationsVariant<'a> {
    PaddockInformations(PaddockInformations<'a>),
    PaddockContentInformations(PaddockContentInformations<'a>),
    PaddockBuyableInformations(PaddockBuyableInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum FightResultListEntryVariant<'a> {
    FightResultListEntry(FightResultListEntry<'a>),
    FightResultFighterListEntry(FightResultFighterListEntry<'a>),
    FightResultMutantListEntry(FightResultMutantListEntry<'a>),
    FightResultPlayerListEntry(FightResultPlayerListEntry<'a>),
    FightResultTaxCollectorListEntry(FightResultTaxCollectorListEntry<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum TreasureHuntStepVariant<'a> {
    TreasureHuntStep(TreasureHuntStep<'a>),
    TreasureHuntStepDig(TreasureHuntStepDig<'a>),
    TreasureHuntStepFight(TreasureHuntStepFight<'a>),
    TreasureHuntStepFollowDirection(TreasureHuntStepFollowDirection<'a>),
    TreasureHuntStepFollowDirectionToHint(TreasureHuntStepFollowDirectionToHint<'a>),
    TreasureHuntStepFollowDirectionToPOI(TreasureHuntStepFollowDirectionToPOI<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PartyMemberInformationsVariant<'a> {
    PartyMemberInformations(PartyMemberInformations<'a>),
    PartyMemberArenaInformations(PartyMemberArenaInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum QuestActiveInformationsVariant<'a> {
    QuestActiveInformations(QuestActiveInformations<'a>),
    QuestActiveDetailedInformations(QuestActiveDetailedInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum TaxCollectorStaticInformationsVariant<'a> {
    TaxCollectorStaticInformations(TaxCollectorStaticInformations<'a>),
    TaxCollectorStaticExtendedInformations(TaxCollectorStaticExtendedInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PartyIdolVariant<'a> {
    PartyIdol(PartyIdol<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum GameRolePlayActorInformationsVariant<'a> {
    GameRolePlayTaxCollectorInformations(GameRolePlayTaxCollectorInformations<'a>),
    GameRolePlayActorInformations(GameRolePlayActorInformations<'a>),
    GameRolePlayNamedActorInformations(GameRolePlayNamedActorInformations<'a>),
    GameRolePlayNpcInformations(GameRolePlayNpcInformations<'a>),
    GameRolePlayNpcWithQuestInformations(GameRolePlayNpcWithQuestInformations<'a>),
    GameRolePlayPortalInformations(GameRolePlayPortalInformations<'a>),
    GameRolePlayPrismInformations(GameRolePlayPrismInformations<'a>),
    GameRolePlayTreasureHintInformations(GameRolePlayTreasureHintInformations<'a>),
    GameRolePlayMerchantInformations(GameRolePlayMerchantInformations<'a>),
    GameRolePlayHumanoidInformations(GameRolePlayHumanoidInformations<'a>),
    GameRolePlayMutantInformations(GameRolePlayMutantInformations<'a>),
    GameRolePlayCharacterInformations(GameRolePlayCharacterInformations<'a>),
    GameRolePlayGroupMonsterInformations(GameRolePlayGroupMonsterInformations<'a>),
    GameRolePlayGroupMonsterWaveInformations(GameRolePlayGroupMonsterWaveInformations<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum GameFightFighterLightInformationsVariant<'a> {
    GameFightFighterLightInformations(GameFightFighterLightInformations<'a>),
    GameFightFighterNamedLightInformations(GameFightFighterNamedLightInformations<'a>),
    GameFightFighterMonsterLightInformations(GameFightFighterMonsterLightInformations<'a>),
    GameFightFighterTaxCollectorLightInformations(
        GameFightFighterTaxCollectorLightInformations<'a>,
    ),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum StatisticDataVariant<'a> {
    StatisticData(StatisticData<'a>),
    StatisticDataBoolean(StatisticDataBoolean<'a>),
    StatisticDataByte(StatisticDataByte<'a>),
    StatisticDataInt(StatisticDataInt<'a>),
    StatisticDataShort(StatisticDataShort<'a>),
    StatisticDataString(StatisticDataString<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct Foo<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum AcquaintanceInformationVariant<'a> {
    Foo(Foo<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum AchievementAchievedVariant<'a> {
    Foo(Foo<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PartyInvitationMemberInformationsVariant<'a> {
    Foo(Foo<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum HouseInformationsVariant<'a> {
    Foo(Foo<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum HouseInstanceInformationsVariant<'a> {
    Foo(Foo<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum UpdateMountCharacteristicVariant<'a> {
    Foo(Foo<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PresetVariant<'a> {
    Foo(Foo<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PartyEntityBaseInformationVariant<'a> {
    Foo(Foo<'a>),
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum PaddockBuyableInformationsVariant<'a> {
    Foo(Foo<'a>),
}
