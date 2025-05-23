use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(PartialEq, Clone, Deserialize)]
pub struct Stats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_flat: f64,
    pub armor_penetration_percent: f64,
    pub attack_damage: f64,
    pub attack_range: f64,
    pub attack_speed: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub current_health: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_resist: f64,
    pub max_health: f64,
    pub max_mana: f64,
    pub current_mana: f64,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct CurrentPlayer {
    pub damaging_abilities: HashMap<String, String>,
    pub damaging_items: HashMap<String, String>,
    pub damaging_runes: HashMap<String, String>,
    pub riot_id: String,
    pub level: usize,
    pub team: String,
    pub position: String,
    pub champion_name: String,
    pub champion_id: String,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct GameInformation {
    pub game_time: f64,
    pub map_number: usize,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct InstanceDamage {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
    pub damage_type: String,
    pub damages_in_area: bool,
    pub damages_onhit: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_dmg_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dmg_change: Option<f64>,
}

pub type DamageLike = HashMap<String, InstanceDamage>;

#[derive(PartialEq, Clone, Deserialize)]
pub struct Damages {
    pub abilities: DamageLike,
    pub items: DamageLike,
    pub runes: DamageLike,
    pub compared_items: HashMap<String, SimulatedDamages>,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct ComparedDamage {
    pub total: f64,
    pub change: f64,
    pub damages: DamageLike,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct SimulatedDamages {
    pub abilities: ComparedDamage,
    pub items: ComparedDamage,
    pub runes: ComparedDamage,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct DragonMultipliers {
    pub earth: f64,
    pub fire: f64,
    pub chemtech: f64,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct Enemy {
    pub champion_id: String,
    pub champion_name: String,
    pub riot_id: String,
    pub team: String,
    pub level: usize,
    pub position: String,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct ItemCompared {
    pub name: String,
    pub gold_cost: usize,
    pub prettified_stats: HashMap<String, Value>,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct Scoreboard {
    pub assists: usize,
    pub creep_score: usize,
    pub deaths: usize,
    pub kills: usize,
    pub riot_id: String,
    pub champion_id: String,
    pub champion_name: String,
    pub team: String,
    pub position: String,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct Realtime {
    pub current_player: CurrentPlayer,
    pub enemies: Vec<Enemy>,
    pub game_information: GameInformation,
    pub recommended_items: Vec<usize>,
    pub compared_items: HashMap<String, ItemCompared>,
    pub scoreboard: Vec<Scoreboard>,
    pub best_item: usize,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
