use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::realtime::{BasicStats, ComparedItem, Damages, RealResists, Stats};

#[derive(Deserialize)]
pub struct CurrentPlayerX {
    pub damaging_abilities: HashMap<String, String>,
    pub damaging_items: HashMap<usize, String>,
    pub damaging_runes: HashMap<usize, String>,
    pub level: usize,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Deserialize)]
pub struct EnemyX {
    pub champion_id: String,
    pub level: usize,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_resists: RealResists,
}

#[derive(Deserialize)]
pub struct Calculator {
    pub current_player: CurrentPlayerX,
    pub enemies: Vec<EnemyX>,
    pub recommended_items: Vec<usize>,
    pub compared_items: HashMap<usize, ComparedItem>,
    pub best_item: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct AbilitiesX {
    pub q: usize,
    pub w: usize,
    pub e: usize,
    pub r: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
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

impl ActivePlayerX {
    pub fn new() -> ActivePlayerX {
        ActivePlayerX {
            champion_id: String::from("Neeko"),
            champion_stats: Stats::default(),
            abilities: AbilitiesX {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            items: Vec::from([4645, 3115, 3153]),
            runes: Vec::new(),
            level: 18,
            stacks: None,
        }
    }
}

#[derive(PartialEq, Clone, Serialize)]
pub struct EnemyPlayersX {
    pub champion_id: String,
    pub items: Vec<usize>,
    pub level: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<BasicStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacks: Option<usize>,
}

#[derive(PartialEq, Clone, Serialize)]
pub struct GameX {
    pub active_player: ActivePlayerX,
    pub enemy_players: Vec<EnemyPlayersX>,
    pub ally_earth_dragons: usize,
    pub ally_fire_dragons: usize,
    pub enemy_earth_dragons: usize,
}
