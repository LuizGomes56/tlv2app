#[macro_export]
macro_rules! apply_stat {
    ($state:expr, $enum_val:expr, $value:expr) => {
        match $enum_val {
            StatsValue::Level(_) => $state.level = $value as usize,
            StatsValue::AttackDamage(_) => $state.champion_stats.attack_damage = $value,
            StatsValue::AbilityPower(_) => $state.champion_stats.ability_power = $value,
            StatsValue::MaxHealth(_) => $state.champion_stats.max_health = $value,
            StatsValue::CurrentHealth(_) => $state.champion_stats.current_health = $value,
            StatsValue::Armor(_) => $state.champion_stats.armor = $value,
            StatsValue::ArmorPenetrationFlat(_) => {
                $state.champion_stats.armor_penetration_flat = $value
            }
            StatsValue::ArmorPenetrationPercent(_) => {
                $state.champion_stats.armor_penetration_percent = $value
            }
            StatsValue::AttackSpeed(_) => $state.champion_stats.attack_speed = $value,
            StatsValue::MagicResist(_) => $state.champion_stats.magic_resist = $value,
            StatsValue::MagicPenetrationFlat(_) => {
                $state.champion_stats.magic_penetration_flat = $value
            }
            StatsValue::MagicPenetrationPercent(_) => {
                $state.champion_stats.magic_penetration_percent = $value
            }
            StatsValue::CritChance(_) => $state.champion_stats.crit_chance = $value,
            StatsValue::CritDamage(_) => $state.champion_stats.crit_damage = $value,
            StatsValue::MaxMana(_) => $state.champion_stats.max_mana = $value,
            StatsValue::CurrentMana(_) => $state.champion_stats.current_mana = $value,
        }
    };
}
