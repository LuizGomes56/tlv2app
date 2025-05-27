use std::ops::Deref;

use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    IMG_CDN, apply_stat,
    components::{
        base_table::BaseTable,
        comparison_table::ComparisonTable,
        stacker::{StackDropper, StackInstance, StackSelector, Stacker},
    },
    model::calculator::{ActivePlayerX, Calculator, CurrentPlayerX, EnemyPlayersX, EnemyX, GameX},
};

#[derive(PartialEq, Properties)]
pub struct AbilityLevelSelectorProps {
    pub state_handler: UseStateHandle<ActivePlayerX>,
    pub keyname: &'static str,
    pub image_url: String,
    pub value: usize,
}

#[function_component]
fn AbilityLevelSelector(props: &AbilityLevelSelectorProps) -> Html {
    let keyname = props.keyname;
    let value = props.value;
    let oninput = {
        let state_handler = props.state_handler.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut current_state = (*state_handler).clone();
            if let Some(input_value) = input.value().parse::<usize>().ok() {
                match keyname {
                    "Q" => current_state.abilities.q = input_value,
                    "W" => current_state.abilities.w = input_value,
                    "E" => current_state.abilities.e = input_value,
                    "R" => current_state.abilities.r = input_value,
                    _ => (),
                }
            }
            state_handler.set(current_state);
        })
    };

    html! {
        <div class="grid grid-cols-[auto_1fr] gap-2">
            <div class="relative flex items-center justify-center">
                <img
                    class="h-8 w-8 aspect-square"
                    src={props.image_url.clone()}
                    alt="Ability"
                />
                <span class="img-letter">{keyname}</span>
            </div>
            <input
                oninput={oninput}
                class="w-full bg-slate-800 h-8 text-center"
                type="number"
                value={value.to_string()}
                min="0"
                max="6"
                aria-label="Ability"
            />
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct StatSelectorProps {
    pub image_url: String,
    pub label_enum: StatsValue,
    pub state_handler: UseStateHandle<ActivePlayerX>,
}

#[function_component]
fn StatSelector(props: &StatSelectorProps) -> Html {
    let cloned_enum = props.label_enum.clone();
    let (name, value) = props.label_enum.get_labels();

    let oninput = {
        let state_handler = props.state_handler.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut current_state = (*state_handler).clone();
            if let Some(input_value) = input.value().parse::<f64>().ok() {
                apply_stat!(current_state, cloned_enum, input_value);
            }
            state_handler.set(current_state);
        })
    };

    html! {
        <>
            <input
                oninput={oninput}
                value={value}
                class="text-sm bg-slate-800 w-16 h-6 text-center"
                type="text"
                aria-label="Ability"
            />
            <img
                class="h-4 w-4 aspect-square"
                src={props.image_url.clone()}
                alt="Ability"
            />
            <span class="text-sm text-shadow">{name}</span>
        </>
    }
}

#[derive(PartialEq, Clone)]
pub enum StatsValue {
    Level(String),
    AbilityPower(String),
    Armor(String),
    ArmorPenetrationFlat(String),
    ArmorPenetrationPercent(String),
    AttackDamage(String),
    AttackSpeed(String),
    CritChance(String),
    CritDamage(String),
    CurrentHealth(String),
    MaxHealth(String),
    MagicPenetrationFlat(String),
    MagicPenetrationPercent(String),
    MagicResist(String),
    MaxMana(String),
    CurrentMana(String),
}

impl StatsValue {
    pub fn get_labels(&self) -> (&'static str, String) {
        match self {
            StatsValue::Level(v) => ("Level", v.clone()),
            StatsValue::CurrentHealth(v) => ("Health", v.clone()),
            StatsValue::AbilityPower(v) => ("Ability Power", v.clone()),
            StatsValue::Armor(v) => ("Armor", v.clone()),
            StatsValue::ArmorPenetrationFlat(v) => ("Armor Pen", v.clone()),
            StatsValue::ArmorPenetrationPercent(v) => ("% Armor Pen", v.clone()),
            StatsValue::AttackDamage(v) => ("Attack Damage", v.clone()),
            StatsValue::AttackSpeed(v) => ("Attack Speed", v.clone()),
            StatsValue::CritChance(v) => ("Crit Chance", v.clone()),
            StatsValue::CritDamage(v) => ("Crit Damage", v.clone()),
            StatsValue::MaxHealth(v) => ("Max Health", v.clone()),
            StatsValue::MagicPenetrationFlat(v) => ("Magic Pen", v.clone()),
            StatsValue::MagicPenetrationPercent(v) => ("% Magic Pen", v.clone()),
            StatsValue::MagicResist(v) => ("Magic Resist", v.clone()),
            StatsValue::MaxMana(v) => ("Max Mana", v.clone()),
            StatsValue::CurrentMana(v) => ("Current Mana", v.clone()),
        }
    }
}

#[derive(Deserialize)]
struct ServerResponse {
    data: Calculator,
}

#[function_component]
pub fn CalculatorDisplay() -> Html {
    let active_player = use_state(|| ActivePlayerX::new());
    let enemy_players = use_state(|| Vec::<EnemyPlayersX>::from([EnemyPlayersX::new()]));
    let calculator_state = use_state(|| Option::<Calculator>::None);
    let stack = use_state(|| Vec::<StackInstance>::new());

    {
        let calculator_state = calculator_state.clone();
        use_effect_with(
            (active_player.clone(), enemy_players.clone()),
            |(active_player, enemy_players)| {
                let game_state = GameX {
                    active_player: active_player.deref().clone(),
                    enemy_players: enemy_players.deref().clone(),
                    ally_earth_dragons: 0,
                    ally_fire_dragons: 0,
                    enemy_earth_dragons: 0,
                };

                spawn_local(async move {
                    let client = reqwest::Client::new();
                    let json_value = json!({
                        "game": game_state,
                        "simulated_items": [3115, 3153, 4645, 3089]
                    });

                    let res = client
                        .post("http://localhost:8082/api/games/calculator")
                        .json(&json_value)
                        .send()
                        .await
                        .unwrap();

                    let result = res.json::<ServerResponse>().await.unwrap();
                    calculator_state.set(Some(result.data));
                });
            },
        );
    }

    html! {
        <div class="flex justify-center">
            <div class="flex flex-col max-h-[calc(100vh-56px)] overflow-y-auto">
                <div class="flex relative">
                    <img
                        class="h-32 img-clipped"
                        src={format!("{}/centered/{}_0.jpg", IMG_CDN, active_player.champion_id)}
                        alt="Banner"
                    />
                </div>
                <div class="flex flex-col gap-4 p-4 bg-slate-900">
                    <div>{ "Champions" }</div>
                    <div>{ "Items" }</div>
                    <div>{ "Runes" }</div>
                </div>
                <section class="grid grid-cols-2 gap-2 py-4 relative">
                    {["Q", "W", "E", "R"].into_iter().map(|ability| {
                        let image_url = format!(
                            "{}/abilities/{}{}.png",
                            IMG_CDN,
                            active_player.champion_id,
                            ability
                        );

                        html! {
                            <AbilityLevelSelector
                                image_url={image_url}
                                state_handler={active_player.clone()}
                                keyname={ability}
                                value={match ability {
                                    "Q" => active_player.abilities.q,
                                    "W" => active_player.abilities.w,
                                    "E" => active_player.abilities.e,
                                    "R" => active_player.abilities.r,
                                    _ => 0
                                }}
                            />
                        }
                    }).collect::<Html>()}
                </section>
                <div class="grid grid-cols-[auto_auto_1fr_auto_auto_1fr] items-center gap-2 pb-8">
                    {[
                        (
                            StatsValue::Level(active_player.level.to_string()),
                            format!("{}/stats/Level.png", IMG_CDN)
                        ),
                        (
                            StatsValue::AbilityPower(active_player.champion_stats.ability_power.to_string()),
                            format!("{}/stats/AbilityPower.png", IMG_CDN)
                        ),
                        (
                            StatsValue::AttackDamage(active_player.champion_stats.attack_damage.to_string()),
                            format!("{}/stats/AttackDamage.png", IMG_CDN)
                        ),
                        (
                            StatsValue::ArmorPenetrationFlat(active_player.champion_stats.armor_penetration_flat.to_string()),
                            format!("{}/stats/ArmorPenetration.png", IMG_CDN)
                        ),
                        (
                            StatsValue::ArmorPenetrationPercent(active_player.champion_stats.armor_penetration_percent.to_string()),
                            format!("{}/stats/ArmorPenetration.png", IMG_CDN)
                        ),
                        (
                            StatsValue::MagicPenetrationFlat(active_player.champion_stats.magic_penetration_flat.to_string()),
                            format!("{}/stats/MagicPenetration.png", IMG_CDN)
                        ),
                        (
                            StatsValue::MagicPenetrationPercent(active_player.champion_stats.magic_penetration_percent.to_string()),
                            format!("{}/stats/MagicPenetration.png", IMG_CDN)
                        ),
                        (
                            StatsValue::CritChance(active_player.champion_stats.crit_chance.to_string()),
                            format!("{}/stats/CriticalStrikeChance.png", IMG_CDN)
                        ),
                        (
                            StatsValue::CritDamage(active_player.champion_stats.crit_damage.to_string()),
                            format!("{}/stats/CriticalStrikeDamage.png", IMG_CDN)
                        ),
                        (
                            StatsValue::MaxHealth(active_player.champion_stats.max_health.to_string()),
                            format!("{}/stats/Health.png", IMG_CDN)
                        ),
                        (
                            StatsValue::CurrentHealth(active_player.champion_stats.max_health.to_string()),
                            format!("{}/stats/Health.png", IMG_CDN)
                        ),
                        (
                            StatsValue::Armor(active_player.champion_stats.armor.to_string()),
                            format!("{}/stats/Armor.png", IMG_CDN)
                        ),
                        (
                            StatsValue::MagicResist(active_player.champion_stats.magic_resist.to_string()),
                            format!("{}/stats/MagicResist.png", IMG_CDN)
                        ),
                        (
                            StatsValue::AttackSpeed(active_player.champion_stats.attack_speed.to_string()),
                            format!("{}/stats/AttackSpeed.png", IMG_CDN)
                        ),
                        (
                            StatsValue::MaxMana(active_player.champion_stats.max_mana.to_string()),
                            format!("{}/stats/Mana.png", IMG_CDN)
                        ),
                        (
                            StatsValue::CurrentMana(active_player.champion_stats.current_mana.to_string()),
                            format!("{}/stats/Mana.png", IMG_CDN)
                        ),
                    ].into_iter().map(|(label_enum, image_url)| {
                        html! {
                            <StatSelector
                                image_url={image_url}
                                label_enum={label_enum}
                                state_handler={active_player.clone()}
                            />
                        }
                    }).collect::<Html>()}
                </div>
            </div>
            {
                if let Some(calculator_data) = (*calculator_state).clone() {
                    let current_player = calculator_data.current_player.clone();
                    let enemies = calculator_data.enemies.clone();

                    html! {
                        <div class="flex flex-col gap-4 p-4 max-w-3xl w-full">
                            <div class="overflow-auto">
                                <BaseTable<CurrentPlayerX, EnemyX>
                                    current_player={current_player.clone()}
                                    enemies={enemies.clone()}
                                />
                            </div>
                            <div class="overflow-auto">
                                <ComparisonTable<CurrentPlayerX, EnemyX>
                                    current_player={current_player.clone()}
                                    enemies={enemies.clone()}
                                    item_id={calculator_data.best_item.clone().to_string()}
                                />
                            </div>
                            <div class="p-4 grid grid-cols-[1fr_auto] gap-4 shadow-container bg-slate-900">
                                <div class="flex flex-col gap-4">
                                    <StackSelector
                                        stack={stack.clone()}
                                        champion_id={current_player.champion_id.clone()}
                                        abilities={current_player.damaging_abilities.clone()}
                                        items={current_player.damaging_items.clone()}
                                        runes={current_player.damaging_runes.clone()}
                                    />
                                    <StackDropper
                                        champion_id={current_player.champion_id.clone()}
                                        stack={stack.clone()}
                                    />
                                </div>
                                <div class="overflow-auto">
                                    <Stacker<EnemyX>
                                        stack={(*stack).clone()}
                                        enemies={enemies.clone()}
                                    />
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
