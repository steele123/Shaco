use std::{fmt, str::FromStr};

use derive_more::Display;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::DeserializeFromStr;

pub type SummonerName = String;
pub type Time = f64;

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct AllGameData {
    /// only available in live game - None in spectator mode
    pub active_player: Option<ActivePlayer>,
    pub all_players: Vec<Player>,
    #[serde(deserialize_with = "serde_single_key_map::deserialize")]
    pub events: Vec<GameEvent>,
    pub game_data: GameStats,
}

impl<'de> Deserialize<'de> for AllGameData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "camelCase")]
        struct Holder {
            /// only available in live game - None in spectator mode
            active_player: ActivePlayerInfo,
            all_players: Vec<Player>,
            #[serde(deserialize_with = "serde_single_key_map::deserialize")]
            events: Vec<GameEvent>,
            game_data: GameStats,
        }
        let holder = Holder::deserialize(deserializer)?;
        let active_player = match holder.active_player {
            ActivePlayerInfo::ActivePlayer(info) => Some(*info),
            ActivePlayerInfo::Error { .. } => None,
        };
        Ok(Self {
            active_player,
            all_players: holder.all_players,
            events: holder.events,
            game_data: holder.game_data,
        })
    }
}

pub type Gold = f32;
pub type Level = i32;

/// only pub(crate) since this is an intermediate result. The API only returns the ActivePlayer struct \
/// only available in live games - is Error when spectating
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ActivePlayerInfo {
    ActivePlayer(Box<ActivePlayer>),
    Error { error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ActivePlayer {
    pub abilities: PlayerAbilities,
    pub champion_stats: PlayerChampionStats,
    pub current_gold: Gold,
    #[serde(alias = "fullRunes")]
    pub runes: FullPlayerRunes,
    pub level: Level,
    pub summoner_name: SummonerName,
    pub team_relative_colors: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerAbilities {
    pub e: Ability,
    pub passive: Passive,
    pub q: Ability,
    pub r: Ability,
    pub w: Ability,
}

pub type AbilityLevel = i32;
pub type AbilityName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub ability_level: AbilityLevel,
    pub display_name: AbilityName,
    pub id: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Passive {
    pub display_name: AbilityName,
    pub id: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

pub type AbilityHaste = f32;
pub type AbilityPower = f32;
pub type Armor = f32;
pub type ArmorPenetrationFlat = f32;
pub type ArmorPenetrationPercent = f32;
pub type AttackDamage = f32;
pub type AttackRange = f32;
pub type AttackSpeed = f32;
pub type BonusArmorPenetrationPercent = f32;
pub type BonusMagicPenetrationPercent = f32;
pub type CritChance = f32;
pub type CritDamage = f32;
pub type CurrentHealth = f32;
pub type HealShieldPower = f32;
pub type HealthRegenRate = f32;
pub type LifeSteal = f32;
pub type MagicLethality = f32;
pub type MagicPenetrationFlat = f32;
pub type MagicPenetrationPercent = f32;
pub type MagicResist = f32;
pub type MaxHealth = f32;
pub type MoveSpeed = f32;
pub type Omnivamp = f32;
pub type PhysicalLethality = f32;
pub type PhysicalVamp = f32;
pub type ResourceMax = f32;
pub type ResourceRegenRate = f32;
pub type ResourceValue = f32;
pub type SpellVamp = f32;
pub type Tenacity = f32;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PlayerChampionStats {
    pub ability_haste: AbilityHaste,
    pub ability_power: AbilityPower,
    pub armor: Armor,
    pub armor_penetration_flat: ArmorPenetrationFlat,
    pub armor_penetration_percent: ArmorPenetrationPercent,
    pub attack_damage: AttackDamage,
    pub attack_range: AttackRange,
    pub attack_speed: AttackSpeed,
    pub bonus_armor_penetration_percent: BonusArmorPenetrationPercent,
    pub bonus_magic_penetration_percent: BonusMagicPenetrationPercent,
    pub crit_chance: CritChance,
    pub crit_damage: CritDamage,
    pub current_health: CurrentHealth,
    pub heal_shield_power: HealShieldPower,
    pub health_regen_rate: HealthRegenRate,
    pub life_steal: LifeSteal,
    pub magic_lethality: MagicLethality,
    pub magic_penetration_flat: MagicPenetrationFlat,
    pub magic_penetration_percent: MagicPenetrationPercent,
    pub magic_resist: MagicResist,
    pub max_health: MaxHealth,
    pub move_speed: MoveSpeed,
    pub omnivamp: Omnivamp,
    pub physical_lethality: PhysicalLethality,
    pub physical_vamp: PhysicalVamp,
    pub resource_max: ResourceMax,
    pub resource_regen_rate: ResourceRegenRate,
    pub resource_type: ResourceType,
    pub resource_value: ResourceValue,
    pub spell_vamp: SpellVamp,
    pub tenacity: Tenacity,
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceType {
    Mana,
    Energy,
    None,
    Shield,
    Battlefury,
    Dragonfury,
    Rage,
    Heat,
    Gnarfury,
    Ferocity,
    Bloodwell,
    Wind,
    Ammo,
    Moonlight,
    Other,
    Max,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct FullPlayerRunes {
    pub general_runes: Vec<Rune>,
    pub keystone: Rune,
    pub primary_rune_tree: RuneTree,
    pub secondary_rune_tree: RuneTree,
    pub stat_runes: Vec<StatRune>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRunes {
    pub keystone: Rune,
    pub primary_rune_tree: RuneTree,
    pub secondary_rune_tree: RuneTree,
}

pub type RuneId = i32;
pub type RuneName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub display_name: RuneName,
    pub id: RuneId,
    pub raw_description: String,
    pub raw_display_name: String,
}

pub type RuneTreeId = i32;
pub type RuneTreeName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RuneTree {
    pub display_name: RuneTreeName,
    pub id: RuneTreeId,
    pub raw_description: String,
    pub raw_display_name: String,
}

pub type StatRuneId = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StatRune {
    pub id: StatRuneId,
    pub raw_description: String,
}

pub type ChampionName = String;
pub type SkinName = String;
pub type SkinId = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub champion_name: ChampionName,
    pub is_bot: bool,
    pub is_dead: bool,
    pub items: Vec<PlayerItem>,
    pub level: Level,
    pub position: Position,
    pub raw_champion_name: String,
    pub respawn_timer: Time,
    pub runes: PlayerRunes,
    pub scores: PlayerScores,
    /// only available in live game - None in spectator mode
    pub raw_skin_name: Option<String>,
    /// only available in live game - None in spectator mode
    pub skin_name: Option<SkinName>,
    #[serde(alias = "skinID")]
    pub skin_id: SkinId,
    pub summoner_name: SummonerName,
    pub summoner_spells: SummonerSpells,
    pub team: TeamId,
}

pub type ItemCount = i32;
pub type ItemName = String;
pub type ItemId = i32;
pub type Price = i32;
pub type ItemSlot = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PlayerItem {
    pub can_use: bool,
    pub consumable: bool,
    pub count: ItemCount,
    pub display_name: ItemName,
    #[serde(alias = "itemID")]
    pub item_id: ItemId,
    pub price: Price,
    pub raw_description: String,
    pub raw_display_name: String,
    pub slot: ItemSlot,
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "UPPERCASE")]
pub enum Position {
    Top,
    Jungle,
    Middle,
    Bottom,
    Utility,
    None,
    #[serde(other)]
    Unknown,
}

pub type Kills = i32;
pub type Deaths = i32;
pub type Assists = i32;
pub type CreepScore = i32;
pub type WardScore = f32;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PlayerScores {
    pub kills: Kills,
    pub deaths: Deaths,
    pub assists: Assists,
    pub creep_score: CreepScore,
    pub ward_score: WardScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpells {
    pub summoner_spell_one: SummonerSpell,
    pub summoner_spell_two: SummonerSpell,
}

pub type SummonerSpellName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
    pub display_name: SummonerSpellName,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "UPPERCASE")]
pub enum TeamId {
    All,
    /// Blue / Left Side
    Order,
    /// Red / Right Side
    Chaos,
    Neutral,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for TeamId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let uppercase = format!("{:?}", self).to_uppercase();
        write!(f, "{uppercase}")
    }
}

pub type EventId = u32;

/// only pub(crate) since this is an intermediate result. The API only returns the Vec<GameEvent>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IngameEvents {
    pub events: Vec<GameEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "EventName")]
pub enum GameEvent {
    Ace(Ace),
    BaronKill(BaronKill),
    ChampionKill(ChampionKill),
    DragonKill(DragonKill),
    FirstBlood(FirstBlood),
    FirstBrick(FirstBrick),
    GameEnd(GameEnd),
    GameStart(GameStart),
    HeraldKill(HeraldKill),
    InhibKilled(InhibKilled),
    InhibRespawned(InhibRespawned),
    InhibRespawningSoon(InhibRespawningSoon),
    MinionsSpawning(MinionsSpawning),
    Multikill(Multikill),
    TurretKilled(TurretKilled),
}

/// event_id and event_time are the only fields all enum variants have in common
impl GameEvent {
    pub fn get_event_id(&self) -> EventId {
        match self {
            GameEvent::Ace(e) => e.event_id,
            GameEvent::BaronKill(e) => e.event_id,
            GameEvent::ChampionKill(e) => e.event_id,
            GameEvent::DragonKill(e) => e.event_id,
            GameEvent::FirstBlood(e) => e.event_id,
            GameEvent::FirstBrick(e) => e.event_id,
            GameEvent::GameEnd(e) => e.event_id,
            GameEvent::GameStart(e) => e.event_id,
            GameEvent::HeraldKill(e) => e.event_id,
            GameEvent::InhibKilled(e) => e.event_id,
            GameEvent::InhibRespawned(e) => e.event_id,
            GameEvent::InhibRespawningSoon(e) => e.event_id,
            GameEvent::MinionsSpawning(e) => e.event_id,
            GameEvent::Multikill(e) => e.event_id,
            GameEvent::TurretKilled(e) => e.event_id,
        }
    }

    pub fn get_event_time(&self) -> Time {
        match self {
            GameEvent::Ace(e) => e.event_time,
            GameEvent::BaronKill(e) => e.event_time,
            GameEvent::ChampionKill(e) => e.event_time,
            GameEvent::DragonKill(e) => e.event_time,
            GameEvent::FirstBlood(e) => e.event_time,
            GameEvent::FirstBrick(e) => e.event_time,
            GameEvent::GameEnd(e) => e.event_time,
            GameEvent::GameStart(e) => e.event_time,
            GameEvent::HeraldKill(e) => e.event_time,
            GameEvent::InhibKilled(e) => e.event_time,
            GameEvent::InhibRespawned(e) => e.event_time,
            GameEvent::InhibRespawningSoon(e) => e.event_time,
            GameEvent::MinionsSpawning(e) => e.event_time,
            GameEvent::Multikill(e) => e.event_time,
            GameEvent::TurretKilled(e) => e.event_time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct Ace {
    pub acer: SummonerName,
    pub acing_team: TeamId,
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct BaronKill {
    pub assisters: Vec<SummonerName>,
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub killer_name: Killer,
    #[serde(deserialize_with = "deserialize_bool")]
    pub stolen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct ChampionKill {
    pub assisters: Vec<SummonerName>,
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub killer_name: Killer,
    pub victim_name: SummonerName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct DragonKill {
    pub assisters: Vec<String>,
    pub dragon_type: DragonType,
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub killer_name: Killer,
    #[serde(deserialize_with = "deserialize_bool")]
    pub stolen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct FirstBlood {
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub recipient: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct FirstBrick {
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub killer_name: Killer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct GameEnd {
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub result: GameResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct GameStart {
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct HeraldKill {
    pub assisters: Vec<String>,
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub killer_name: Killer,
    #[serde(deserialize_with = "deserialize_bool")]
    pub stolen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct InhibKilled {
    pub assisters: Vec<String>,
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub inhib_killed: Inhibitor,
    pub killer_name: Killer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct InhibRespawned {
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub inhib_respawned: Inhibitor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct InhibRespawningSoon {
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub inhib_respawning_soon: Inhibitor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct MinionsSpawning {
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
}

pub type KillStreak = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct Multikill {
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub kill_streak: KillStreak,
    pub killer_name: SummonerName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct TurretKilled {
    pub assisters: Vec<String>,
    #[serde(rename = "EventID")]
    pub event_id: EventId,
    pub event_time: Time,
    pub killer_name: Killer,
    pub turret_killed: Turret,
}

#[derive(Debug, Display, Clone, Serialize, DeserializeFromStr)]
#[serde(deny_unknown_fields)]
pub enum DragonType {
    Infernal,
    Ocean,
    Mountain,
    Cloud,
    Hextech,
    Chemtech,
    Elder,
}

impl FromStr for DragonType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dragon_type = match s {
            "Fire" | "SRU_Dragon_Fire" => DragonType::Infernal,
            "Water" | "SRU_Dragon_Water" => DragonType::Ocean,
            "Earth" | "SRU_Dragon_Earth" => DragonType::Mountain,
            "Air" | "SRU_Dragon_Air" => DragonType::Cloud,
            "Hextech" | "SRU_Dragon_Hextech" => DragonType::Hextech,
            "Chemtech" | "SRU_Dragon_Chemtech" => DragonType::Chemtech,
            "Elder" | "SRU_Dragon_Elder" => DragonType::Elder,
            _ => return Err(s.to_string()),
        };
        Ok(dragon_type)
    }
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum GameResult {
    Win,
    Lose,
}

#[derive(Debug, Display, Clone, Serialize, DeserializeFromStr)]
#[serde(deny_unknown_fields)]
pub enum Killer {
    Minion,
    Dragon(DragonType),
    Gromp,
    Blue,
    Murkwolf,
    Razorbeak,
    Red,
    Krug,
    RiftHerald,
    Baron,
    Turret(Turret),
    Summoner(SummonerName),
}

impl FromStr for Killer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // SRU_Dragon_{type}
        if let Ok(dragon) = DragonType::from_str(s) {
            return Ok(Killer::Dragon(dragon));
        }
        // Turret_{}_{}_{}_{}
        if let Ok(turret) = Turret::from_str(s) {
            return Ok(Killer::Turret(turret));
        }
        // Minion_{}
        if s.starts_with("Minion") {
            return Ok(Killer::Minion);
        }
        // SRU_RiftHerald{}
        if s.contains("RiftHerald") {
            return Ok(Killer::RiftHerald);
        }
        // SRU_Baron{}
        if s.contains("Baron") {
            return Ok(Killer::RiftHerald);
        }
        // SRU_Gromp{}
        if s.contains("Gromp") {
            return Ok(Killer::Gromp);
        }
        // SRU_Blue{}
        if s.contains("Blue") {
            return Ok(Killer::Blue);
        }
        // SRU_Murkwolf{}
        if s.contains("Murkwolf") {
            return Ok(Killer::Murkwolf);
        }
        // SRU_Razorbeak{}
        if s.contains("Razorbeak") {
            return Ok(Killer::Razorbeak);
        }
        // SRU_Red{}
        if s.contains("Red") {
            return Ok(Killer::Red);
        }
        // SRU_Krug{}
        if s.contains("Krug") {
            return Ok(Killer::Krug);
        }

        // if none of the others its 99% the summoner name
        Ok(Killer::Summoner(s.to_string()))
    }
}

/// Team 1 refers to Blue / Left Side \
/// Team 2 refers to Red / Right Side
#[derive(Debug, Display, Clone, Serialize, DeserializeFromStr)]
#[serde(deny_unknown_fields)]
pub enum Turret {
    // --- TEAM1 ---
    /// *Summoner's Rift*: Team 1 Upper Nexus Turret
    Team1C01A,
    /// *Summoner's Rift*: Team 1 Lower Nexus Turret
    Team1C02A,
    /// *Summoner's Rift*: Team 1 Mid Inhib Turret
    Team1C03A,
    /// *Summoner's Rift*: Team 1 Mid Inner Turret
    Team1C04A,
    /// *Summoner's Rift*: Team 1 Mid Outer Turret
    Team1C05A,
    /// *Summoner's Rift*: Team 1 Top Inihb Turret
    Team1C06A,
    /// *Summoner's Rift*: Team 1 Bot Inhib Turret \
    /// *ARAM*: Team 1 Inner Turret
    Team1C07A,
    /// *ARAM*: Team 1 Outer Turret
    Team1C08A,
    /// *ARAM*: Team 1 Bot Nexus Turret
    Team1C09A,
    /// *ARAM*: Team 1 Top Nexus Turret
    Team1C10A,
    /// *Summoner's Rift*: Team 1 Top Inner Turret
    Team1L02A,
    /// *Summoner's Rift*: Team 1 Top Outer Turret
    Team1L03A,
    /// *Summoner's Rift*: Team 1 Bot Inner Turret
    Team1R02A,
    /// *Summoner's Rift*: Team 1 Bot Outer Turret
    Team1R03A,
    /// *Summoner's Rift*: Team 1 Fountain
    /// *ARAM*: Team 1 Fountain
    Team1Fountain,
    // --- TEAM2 ---
    /// *Summoner's Rift*: Team 2 Lower Nexus Turret
    Team2C01A,
    /// *Summoner's Rift*: Team 2 Upper Nexus Turret
    Team2C02A,
    /// *Summoner's Rift*: Team 2 Mid Inhib Turret
    Team2C03A,
    /// *Summoner's Rift*: Team 2 Mid Inner Turret
    Team2C04A,
    /// *Summoner's Rift*: Team 2 Mid Outer Turret
    Team2C05A,
    /// *Summoner's Rift*: Team 2 Top Inihb Turret
    /// *ARAM*: Team 2 Outer Turret
    Team2L01A,
    /// *Summoner's Rift*: Team 2 Top Inner Turret
    /// *ARAM*: Team 2 Inner Turret
    Team2L02A,
    /// *Summoner's Rift*: Team 2 Top Outer Turret
    /// *ARAM*: Team 2 Lower Nexus Turret
    Team2L03A,
    /// *ARAM*: Team 2 Upper Nexus Turret
    Team2L04A,
    /// *Summoner's Rift*: Team 2 Bot Inhib Turret
    Team2R01A,
    /// *Summoner's Rift*: Team 2 Bot Inner Turret
    Team2R02A,
    /// *Summoner's Rift*: Team 2 Bot Outer Turret
    Team2R03A,
    /// *Summoner's Rift*: Team 2 Fountain
    /// *ARAM*: Team 2 Fountain
    Team2Fountain,
    // --- OTHER ---
    /// Azir Turret
    Obelisk,
    /// The riot documentation specifying the turrets is incomplete => add Unknown to catch deserialization errors
    #[serde(other)]
    Unknown,
}

impl FromStr for Turret {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let turret = match s {
            // --- TEAM1 ---
            "Turret_T1_C_01_A" => Turret::Team1C01A,
            "Turret_T1_C_02_A" => Turret::Team1C02A,
            "Turret_T1_C_03_A" => Turret::Team1C03A,
            "Turret_T1_C_04_A" => Turret::Team1C04A,
            "Turret_T1_C_05_A" => Turret::Team1C05A,
            "Turret_T1_C_06_A" => Turret::Team1C06A,
            "Turret_T1_C_07_A" => Turret::Team1C07A,
            "Turret_T1_C_08_A" => Turret::Team1C08A,
            "Turret_T1_C_09_A" => Turret::Team1C09A,
            "Turret_T1_C_010_A" => Turret::Team1C10A,
            "Turret_T1_L_02_A" => Turret::Team1L02A,
            "Turret_T1_L_03_A" => Turret::Team1L03A,
            "Turret_T1_R_02_A" => Turret::Team1R02A,
            "Turret_T1_R_03_A" => Turret::Team1R03A,
            "Turret_OrderTurretShrine_A" => Turret::Team1Fountain,
            // --- TEAM2 ---
            "Turret_T2_C_01_A" => Turret::Team2C01A,
            "Turret_T2_C_02_A" => Turret::Team2C02A,
            "Turret_T2_C_03_A" => Turret::Team2C03A,
            "Turret_T2_C_04_A" => Turret::Team2C04A,
            "Turret_T2_C_05_A" => Turret::Team2C05A,
            "Turret_T2_L_01_A" => Turret::Team2L01A,
            "Turret_T2_L_02_A" => Turret::Team2L02A,
            "Turret_T2_L_03_A" => Turret::Team2L03A,
            "Turret_T2_L_04_A" => Turret::Team2L04A,
            "Turret_T2_R_01_A" => Turret::Team2R01A,
            "Turret_T2_R_02_A" => Turret::Team2R02A,
            "Turret_T2_R_03_A" => Turret::Team2R03A,
            "Turret_ChaosTurretShrine_A" => Turret::Team2Fountain,
            // --- OTHER ---
            "Obelisk" => Turret::Obelisk,
            _ => Turret::Unknown,
        };
        Ok(turret)
    }
}

/// Team 1 refers to Blue / Left Side \
/// Team 2 refers to Red / Right Side
#[derive(Debug, Display, Clone, Serialize, DeserializeFromStr)]
#[serde(deny_unknown_fields)]
pub enum Inhibitor {
    /// *Summoner's Rift*: Team 1 Top Inhibitor
    Team1L1,
    /// *Summoner's Rift*: Team 1 Mid Inhibitor \
    /// **ARAM**: Team 1 Inhibitor
    Team1C1,
    /// *Summoner's Rift*: Team 1 Bot Inhibitor
    Team1R1,
    /// *Summoner's Rift*: Team 2 Top Inhibitor
    Team2L1,
    /// *Summoner's Rift*: Team 1 Mid Inhibitor \
    /// **ARAM**: Team 2 Inhibitor
    Team2C1,
    /// *Summoner's Rift*: Team 2 Bot Inhibitor
    Team2R1,
    #[serde(other)]
    Unknown,
}

impl FromStr for Inhibitor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inhib = match s {
            "Barracks_T1_L1" => Inhibitor::Team1L1,
            "Barracks_T1_C1" => Inhibitor::Team1C1,
            "Barracks_T1_R1" => Inhibitor::Team1R1,
            "Barracks_T2_L1" => Inhibitor::Team2L1,
            "Barracks_T2_C1" => Inhibitor::Team2C1,
            "Barracks_T2_R1" => Inhibitor::Team2R1,
            _ => Inhibitor::Unknown,
        };
        Ok(inhib)
    }
}

fn deserialize_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let string = Deserialize::deserialize(deserializer)?;
    Ok(match string {
        "True" => true,
        "False" => false,
        _ => panic!("unknown value for bool"),
    })
}

pub type MapNumber = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct GameStats {
    pub game_mode: GameMode,
    pub game_time: Time,
    pub map_name: MapName,
    pub map_number: MapNumber,
    pub map_terrain: MapTerrain,
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "UPPERCASE")]
pub enum GameMode {
    /// Classic Summoner's Rift and Twisted Treeline games
    Classic,
    /// Dominion/Crystal Scar games
    Odin,
    Aram,
    Tutorial,
    /// Part 1 of the tutorial
    #[serde(alias = "TUTORIAL_MODULE_1")]
    Tutorial1,
    /// Part 2 of the tutorial
    #[serde(alias = "TUTORIAL_MODULE_2")]
    Tutorial2,
    /// Part 3 of the tutorial
    #[serde(alias = "TUTORIAL_MODULE_3")]
    Tutorial3,
    Urf,
    PracticeTool,
    /// Doombots games
    DoombotsTeemo,
    OneForAll,
    Ascension,
    /// Snowdown Showdown games
    FirstBlood,
    /// Poroking games
    KingPoro,
    /// Nexus Siege games
    Siege,
    /// Blood Hunt Assassin games
    Assassinate,
    /// All Random Summoner's Rift games
    ARSR,
    /// Dark Star: Singularity games
    Darkstar,
    /// Star Guardian Invasion games
    StarGuardian,
    /// PROJECT: Hunters games
    Project,
    #[serde(alias = "GAMEMODEX")]
    NexusBlitz,
    /// Odyssey: Extraction games
    Odyssey,
    /// Ultimate Spellbook
    UltBook,
    /// The riot documentation specifying the game modes is incomplete => add Unknown to catch deserialization errors
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub enum MapName {
    /// Summoner's Rift - Original Summer variant
    Map1,
    /// Summoner's Rift - Original Autumn variant
    Map2,
    /// The Proving Grounds - Tutorial map
    Map3,
    /// Twisted Treeline - Original Version
    Map4,
    /// The Crystal Scar - Dominion map
    Map8,
    /// Twisted Treeline
    Map10,
    /// Summoner's Rift - Current Version
    Map11,
    /// Howling Abyss - ARAM map
    Map12,
    /// Butcher's Bridge: Alternate ARAM map
    Map14,
    /// Cosmic Ruins - Dark Star: Singularity map
    Map16,
    /// Valoran City Park - Star Guardian Invasion map
    Map18,
    /// Substructure 43 - PROJECT: Hunters map
    Map19,
    /// Crash Site - Odyssey: Extraction map
    Map20,
    /// Nexus Blitz
    Map21,
    /// Convergence - Teamfight Tactics map
    Map22,
    /// The riot documentation specifying the maps is incomplete => add Unknown to catch deserialization errors
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum MapTerrain {
    Default,
    Infernal,
    Ocean,
    Mountain,
    Cloud,
    Hextech,
    Chemtech,
}
