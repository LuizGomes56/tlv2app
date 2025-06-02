#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

#[derive(Deserialize)]
pub struct EnemyPlayersX {
    pub champion_id: String,
    pub items: Vec<usize>,
    pub level: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<BasicStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacks: Option<usize>,
}

#[derive(Deserialize)]
pub struct AbilitiesX {
    pub q: usize,
    pub w: usize,
    pub e: usize,
    pub r: usize,
}

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
pub struct ActivePlayerX {
    pub champion_id: String,
    pub champion_stats: Stats,
    pub abilities: AbilitiesX,
    pub items: Vec<usize>,
    pub runes: Vec<usize>,
    pub level: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacks: Option<usize>,
}

#[derive(Deserialize)]
pub struct GameX {
    active_player: ActivePlayerX,
    enemy_players: Vec<EnemyPlayersX>,
    ally_earth_dragons: i32,
    ally_fire_dragons: i32,
    enemy_earth_dragons: i32,
}
