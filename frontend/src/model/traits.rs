use std::collections::HashMap;

use super::{
    calculator::{CurrentPlayerX, EnemyX},
    realtime::{BasicStats, CurrentPlayer, Damages, Enemy},
};

pub trait EnemyLike {
    fn get_damages(&self) -> Damages;
    fn get_champion_id(&self) -> String;
    fn get_champion_name(&self) -> String;
    fn get_current_stats(&self) -> BasicStats;
}

pub trait CurrentPlayerLike {
    fn get_damaging_instances(
        &self,
    ) -> (
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, String>,
    );
    fn get_champion_id(&self) -> String;
}

impl CurrentPlayerLike for CurrentPlayer {
    fn get_damaging_instances(
        &self,
    ) -> (
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, String>,
    ) {
        (
            self.damaging_abilities.clone(),
            self.damaging_items.clone(),
            self.damaging_runes.clone(),
        )
    }

    fn get_champion_id(&self) -> String {
        self.champion_id.clone()
    }
}

impl CurrentPlayerLike for CurrentPlayerX {
    fn get_damaging_instances(
        &self,
    ) -> (
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, String>,
    ) {
        (
            self.damaging_abilities.clone(),
            self.damaging_items.clone(),
            self.damaging_runes.clone(),
        )
    }

    fn get_champion_id(&self) -> String {
        self.champion_id.clone()
    }
}

impl EnemyLike for Enemy {
    fn get_champion_id(&self) -> String {
        self.champion_id.clone()
    }

    fn get_champion_name(&self) -> String {
        self.champion_name.clone()
    }

    fn get_damages(&self) -> Damages {
        self.damages.clone()
    }

    fn get_current_stats(&self) -> BasicStats {
        self.current_stats.clone()
    }
}

impl EnemyLike for EnemyX {
    fn get_champion_id(&self) -> String {
        self.champion_id.clone()
    }

    fn get_champion_name(&self) -> String {
        self.champion_name.clone()
    }

    fn get_damages(&self) -> Damages {
        self.damages.clone()
    }

    fn get_current_stats(&self) -> BasicStats {
        self.current_stats.clone()
    }
}
