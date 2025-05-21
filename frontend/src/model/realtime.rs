use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

#[derive(Deserialize)]
pub struct CurrentPlayer {
    pub damaging_abilities: HashMap<String, String>,
    pub damaging_items: HashMap<usize, String>,
    pub damaging_runes: HashMap<usize, String>,
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

#[derive(Deserialize)]
pub struct GameInformation {
    pub game_time: f64,
    pub map_number: usize,
}

#[derive(Deserialize)]
pub struct InstanceDamage {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
    pub damage_type: String,
    pub damages_in_area: bool,
    pub damages_onhit: bool,
}

#[derive(Deserialize)]
pub struct BasicDamages {
    pub abilities: HashMap<String, InstanceDamage>,
    pub items: HashMap<usize, InstanceDamage>,
    pub runes: HashMap<usize, InstanceDamage>,
}

#[derive(Deserialize)]
pub struct Damages {
    pub abilities: HashMap<String, InstanceDamage>,
    pub items: HashMap<usize, InstanceDamage>,
    pub runes: HashMap<usize, InstanceDamage>,
    pub compared_items: HashMap<String, BasicDamages>,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct ItemCompared {
    pub name: String,
    pub has_active: bool,
    pub gold_cost: usize,
    pub prettified_stats: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct Realtime {
    pub current_player: CurrentPlayer,
    pub enemies: Vec<Enemy>,
    pub game_information: GameInformation,
    pub recommended_items: Vec<usize>,
    pub compared_items: HashMap<usize, ItemCompared>,
}
