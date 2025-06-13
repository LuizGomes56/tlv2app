use std::{collections::HashMap, ops::Deref, rc::Rc};

use reqwasm::http::Request;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    BACKEND_URL, apply_stat,
    components::{
        base_table::base_table,
        comparison_header::comparison_header,
        comparison_table::comparison_table,
        selector::{SelectionMode, Selector},
        stacker::{StackInstance, stack_dropper, stack_selector, stacker},
        value_cell::value_cell,
    },
    context::core::CoreContext,
    model::{
        calculator::{ActivePlayerX, Calculator, CurrentPlayerX, EnemyPlayersX, EnemyX, GameX},
        server::ServerResponse,
        traits::CurrentPlayerLike,
    },
};

fn ability_level_selector(
    state_handler: &UseStateHandle<ActivePlayerX>,
    keyname: &'static str,
    image_url: String,
    value: usize,
) -> Html {
    let oninput = {
        let state_handler = state_handler.clone();
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
        <div class={"grid grid-cols-[auto_1fr] gap-2"}>
            <div class={"relative flex items-center justify-center"}>
                <img
                    class={"h-8 min-w-8 aspect-square"}
                    src={image_url}
                    alt="Ability"
                />
                <span class={"img-letter"}>{keyname}</span>
            </div>
            <input
                oninput={oninput}
                class={"w-full bg-custom-800 h-8 text-center"}
                type="text"
                value={value.to_string()}
                maxlength="1"
                aria-label="Ability"
            />
        </div>
    }
}

fn stat_selector(
    image_url: String,
    label_enum: StatsValue,
    oninput: &Callback<InputEvent>,
) -> Html {
    let (name, value) = label_enum.get_labels();

    html! {
        <>
            <input
                oninput={oninput}
                value={value}
                class={"text-sm bg-custom-800 w-16 h-6 text-center"}
                type="text"
                maxlength="6"
                aria-label="Ability"
            />
            <img
                class={"h-4 min-w-4 aspect-square"}
                src={image_url.clone()}
                alt="Ability"
            />
            <span class={"text-sm text-shadow"}>{name}</span>
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

#[function_component]
pub fn CalculatorDisplay() -> Html {
    let active_player = use_state(|| ActivePlayerX::new());
    let enemy_players = use_state(|| Vec::<EnemyPlayersX>::from([EnemyPlayersX::new(0)]));
    let active_player_stacks = use_state(|| 0usize);
    let ally_earth_dragons = use_state(|| 0usize);
    let ally_fire_dragons = use_state(|| 0usize);
    let enemy_earth_dragons = use_state(|| 0usize);
    let simulated_items = use_state(|| Vec::<usize>::from([3115, 3153, 4645, 3089]));
    let enemy_index = use_state(|| 0usize);
    let calculator_state = use_state(|| Option::<Calculator>::None);
    let stack = use_state(|| Vec::<StackInstance>::new());

    let context = use_context::<CoreContext>().expect("CoreContext not found");

    let all_champions = context.get_static_champions();
    let all_items = context.get_static_items();
    let all_runes = context.get_static_runes();

    let error_occurred = use_state(|| false);

    let onerror_callback = {
        let error_occurred = error_occurred.clone();
        Callback::from(move |_| {
            error_occurred.set(true);
        })
    };

    {
        let calculator_state = calculator_state.clone();
        let ally_earth_dragons = ally_earth_dragons.clone();
        let ally_fire_dragons = ally_fire_dragons.clone();
        let enemy_earth_dragons = enemy_earth_dragons.clone();
        let simulated_items = simulated_items.clone();
        use_effect_with(
            (
                active_player.clone(),
                enemy_players.clone(),
                ally_earth_dragons.clone(),
                ally_fire_dragons.clone(),
                enemy_earth_dragons.clone(),
            ),
            |(
                active_player,
                enemy_players,
                ally_earth_dragons,
                ally_fire_dragons,
                enemy_earth_dragons,
            )| {
                let game_state = GameX {
                    active_player: active_player.deref().clone(),
                    enemy_players: enemy_players.deref().clone(),
                    ally_earth_dragons: ally_earth_dragons.deref().clone(),
                    ally_fire_dragons: ally_fire_dragons.deref().clone(),
                    enemy_earth_dragons: enemy_earth_dragons.deref().clone(),
                    stack_exceptions: HashMap::new(),
                };

                spawn_local(async move {
                    let res = Request::post(&format!("{}/api/games/calculator", BACKEND_URL))
                        .header("Content-Type", "application/json")
                        .body(
                            json!({
                                "game": game_state,
                                "simulated_items": simulated_items.deref().clone()
                            })
                            .to_string(),
                        )
                        .send()
                        .await
                        .unwrap();

                    if let Some(result) = res.json::<ServerResponse<Calculator>>().await.ok() {
                        calculator_state.set(Some(result.data));
                    }
                });
            },
        );
    }

    html! {
        <div class={"h-screen overflow-y-auto grid grid-cols-[min-content_minmax(384px,1fr)_auto] gap-2 px-2 py-4"}>
            <div class={"flex flex-col max-h-screen overflow-y-auto px-2"}>
                <div class={"flex relative"}>
                    <img
                        class={"h-28 img-clipped"}
                        src={if *error_occurred {
                            format!("{}/cdn/splash/{}_0.jpg", BACKEND_URL, active_player.champion_id)
                        } else {
                            format!("{}/cdn/centered/{}_0.jpg", BACKEND_URL, active_player.champion_id)
                        }}
                        onerror={onerror_callback}
                        alt="Banner"
                    />
                </div>
                <div class={"flex flex-col"}>
                    <Selector<String>
                        source_map={all_champions}
                        uri={format!("{}/cdn/champions", BACKEND_URL)}
                        title={"Champion"}
                        selection={SelectionMode::Single({
                            let active_player = active_player.clone();
                            Callback::from(move |champion_id: String| {
                                active_player.set(ActivePlayerX {
                                    champion_id,
                                    ..active_player.deref().clone()
                                })
                            })
                        })}
                    />
                    <Selector<usize>
                        source_map={all_items}
                        uri={format!("{}/cdn/items", BACKEND_URL)}
                        title={"Items"}
                        selection={SelectionMode::Multiple({
                            let active_player = active_player.clone();
                            Callback::from(move |items: Vec<usize>| {
                                active_player.set(ActivePlayerX {
                                    items,
                                    ..active_player.deref().clone()
                                })
                            })
                        })}
                    />
                    <Selector<usize>
                        source_map={all_runes}
                        uri={format!("{}/cdn/runes", BACKEND_URL)}
                        title={"Runes"}
                        selection={SelectionMode::Multiple({
                            let active_player = active_player.clone();
                            Callback::from(move |runes: Vec<usize>| {
                                active_player.set(ActivePlayerX {
                                    runes,
                                    ..active_player.deref().clone()
                                })
                            })
                        })}
                    />
                </div>
                <section class={"grid grid-cols-2 gap-2 py-2"}>
                    {
                        ["Q", "W", "E", "R"].into_iter().map(|ability| {
                            let image_url = format!(
                                "{}/cdn/abilities/{}{}.png",
                                BACKEND_URL,
                                active_player.champion_id,
                                ability
                            );

                            html! {
                                ability_level_selector(
                                    &active_player,
                                    ability,
                                    image_url,
                                    match ability {
                                        "Q" => active_player.abilities.q,
                                        "W" => active_player.abilities.w,
                                        "E" => active_player.abilities.e,
                                        "R" => active_player.abilities.r,
                                        _ => 0
                                    }
                                )
                            }
                        }).collect::<Html>()
                    }
                    {
                        value_cell(
                            "EarthDragon.png",
                            ally_earth_dragons.deref().clone().to_string(),
                            {
                                let ally_earth_dragons = ally_earth_dragons.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    ally_earth_dragons.set(input.value().parse().unwrap_or_default());
                                })
                            },
                        )
                    }
                    {
                        value_cell(
                            "FireDragon.png",
                            ally_fire_dragons.deref().clone().to_string(),
                            {
                                let ally_fire_dragons = ally_fire_dragons.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    ally_fire_dragons.set(input.value().parse().unwrap_or_default());
                                })
                            },
                        )
                    }
                    {
                        value_cell(
                            "stack.svg",
                            active_player_stacks.deref().clone().to_string(),
                            {
                                let active_player_stacks = active_player_stacks.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    active_player_stacks.set(input.value().parse().unwrap_or_default());
                                })
                            },
                        )
                    }
                </section>
                <div class={"grid grid-cols-[auto_auto_1fr] items-center gap-2 pb-8"}>
                    {[
                        (
                            StatsValue::Level(active_player.level.to_string()),
                            format!("{}/cdn/stats/Level.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::AttackSpeed(active_player.champion_stats.attack_speed.to_string()),
                            format!("{}/cdn/stats/AttackSpeed.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::AbilityPower(active_player.champion_stats.ability_power.to_string()),
                            format!("{}/cdn/stats/AbilityPower.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::AttackDamage(active_player.champion_stats.attack_damage.to_string()),
                            format!("{}/cdn/stats/AttackDamage.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::ArmorPenetrationFlat(active_player.champion_stats.armor_penetration_flat.to_string()),
                            format!("{}/cdn/stats/ArmorPenetration.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::ArmorPenetrationPercent(active_player.champion_stats.armor_penetration_percent.to_string()),
                            format!("{}/cdn/stats/ArmorPenetration.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::MagicPenetrationFlat(active_player.champion_stats.magic_penetration_flat.to_string()),
                            format!("{}/cdn/stats/MagicPenetration.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::MagicPenetrationPercent(active_player.champion_stats.magic_penetration_percent.to_string()),
                            format!("{}/cdn/stats/MagicPenetration.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::CritChance(active_player.champion_stats.crit_chance.to_string()),
                            format!("{}/cdn/stats/CriticalStrikeChance.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::CritDamage(active_player.champion_stats.crit_damage.to_string()),
                            format!("{}/cdn/stats/CriticalStrikeDamage.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::MaxHealth(active_player.champion_stats.max_health.to_string()),
                            format!("{}/cdn/stats/Health.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::CurrentHealth(active_player.champion_stats.current_health.to_string()),
                            format!("{}/cdn/stats/Health.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::Armor(active_player.champion_stats.armor.to_string()),
                            format!("{}/cdn/stats/Armor.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::MagicResist(active_player.champion_stats.magic_resist.to_string()),
                            format!("{}/cdn/stats/MagicResist.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::MaxMana(active_player.champion_stats.max_mana.to_string()),
                            format!("{}/cdn/stats/Mana.png", BACKEND_URL)
                        ),
                        (
                            StatsValue::CurrentMana(active_player.champion_stats.current_mana.to_string()),
                            format!("{}/cdn/stats/Mana.png", BACKEND_URL)
                        ),
                    ].into_iter().map(|(label_enum, image_url)| {
                        let oninput = {
                            let active_player = active_player.clone();
                            let label_enum = label_enum.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                let mut current_state = (*active_player).clone();
                                if let Some(input_value) = input.value().parse::<f64>().ok() {
                                    apply_stat!(current_state, label_enum, input_value);
                                }
                                active_player.set(current_state);
                            })
                        };

                        html! {
                            stat_selector(
                                image_url,
                                label_enum,
                                &oninput
                            )
                        }
                    }).collect::<Html>()}
                </div>
            </div>
            {
                if let Some(calculator_data) = (*calculator_state).clone() {
                    let current_player = calculator_data.current_player.clone();
                    let enemies = calculator_data.enemies.clone();

                    let mut compared_items: Vec<_> = calculator_data.compared_items.iter().collect();
                    compared_items.sort_by_key(|(key, _)| *key);

                    html! {
                        <div class={"flex flex-col gap-4 flex-1"}>
                            <div class={"overflow-auto"}>
                                { base_table(&current_player, &enemies) }
                            </div>
                            /*
                            {
                                compared_items.into_iter().map(|(item_id, value)| {
                                    html! {
                                        <div class={"shadow-container bg-custom-900"}>
                                            <div class={"flex flex-col"}>
                                                {
                                                    comparison_header(
                                                        value,
                                                        &item_id.to_string()
                                                    )
                                                }
                                                <div class={"overflow-auto"}>
                                                    {
                                                        comparison_table(
                                                            &current_player,
                                                            &enemies,
                                                            calculator_data.best_item.to_string(),
                                                        )
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                            */
                            <div class={"p-4 grid grid-cols-[1fr_auto] gap-4 shadow-container bg-custom-900"}>
                                <div class={"flex flex-col gap-4"}>
                                    {
                                        stack_selector(
                                            &stack,
                                            current_player.champion_id.clone(),
                                            current_player.get_damaging_instances(),
                                        )
                                    }
                                    {
                                        stack_dropper(
                                            &stack,
                                            Some(current_player.champion_id),
                                        )
                                    }
                                </div>
                                <div class={"overflow-auto"}>
                                    {
                                        stacker(
                                            &stack,
                                            &enemies
                                        )
                                    }
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div>
                            { "Loading or an error might have occured" }
                        </div>
                    }
                }
            }
            <div class={"flex flex-col px-2"}>
                <div class={"grid grid-cols-[auto_1fr_1fr_auto] mb-4 gap-2 h-12 text-center text-lg text-shadow bg-custom-800"}>
                    <div
                        onclick={
                            let enemy_index = enemy_index.clone();
                            let current_index = *enemy_index;
                            Callback::from(move |_| {
                                if current_index > 0 {
                                    enemy_index.set(current_index - 1);
                                }
                            })
                        }
                        class={"flex items-center justify-center aspect-square select-none cursor-pointer flex-shrink-0 font-bold"}>
                        { "<" }
                    </div>
                    {
                        if enemy_players.len() < 5 {
                            let onclick = {
                                let enemy_players = enemy_players.clone();
                                Callback::from(move |_| {
                                    enemy_players.set({
                                        let mut enemy_players_vec = (*enemy_players).clone();
                                        let length = enemy_players_vec.len();
                                        enemy_players_vec.push(EnemyPlayersX::new(length));
                                        enemy_players_vec
                                    });
                                })
                            };

                            html! {
                                <button
                                    onclick={onclick}
                                    class={"select-none cursor-pointer"}>
                                    { "+" }
                                </button>
                            }
                        } else {
                            html! {
                                <div>
                                </div>
                            }
                        }
                    }
                    {
                        if enemy_players.len() > 1 {
                            let onclick = {
                                let enemy_players = enemy_players.clone();
                                Callback::from(move |_| {
                                    enemy_players.set({
                                        let mut enemy_players_vec = (*enemy_players).clone();
                                        enemy_players_vec.pop();
                                        enemy_players_vec
                                    });
                                })
                            };

                            html! {
                                <button
                                    onclick={onclick}
                                    class={"select-none cursor-pointer"}>
                                    { "-" }
                                </button>
                            }
                        } else {
                            html! {
                                <div>
                                </div>
                            }
                        }
                    }
                    <div
                        onclick={
                            let enemy_index = enemy_index.clone();
                            let current_index = *enemy_index;
                            let enemy_players = enemy_players.clone();
                            Callback::from(move |_| {
                                if current_index < enemy_players.len() - 1 {
                                    enemy_index.set(current_index + 1);
                                }
                            })
                        }
                        class={"flex items-center justify-center aspect-square select-none cursor-pointer flex-shrink-0 font-bold"}>
                        { ">" }
                    </div>
                </div>
                {
                    enemy_players.iter().enumerate().map(|(player_index, player)| {
                        let hidden_class = if player_index == *enemy_index { "" } else { "hidden" };

                        html! {
                            <div class={hidden_class}>
                                <div class={"flex relative"}>
                                    <img
                                        class={"h-28 img-clipped"}
                                        src={format!("{}/cdn/centered/{}_0.jpg", BACKEND_URL, player.champion_id)}
                                        alt="Banner"
                                    />
                                </div>
                                <div class={"flex flex-col"}>
                                    <Selector<String>
                                        source_map={all_champions}
                                        uri={format!("{}/cdn/champions", BACKEND_URL)}
                                        title={"Champion"}
                                        selection={SelectionMode::Single({
                                            let enemy_players = enemy_players.clone();
                                            let player_index = player_index;
                                            Callback::from(move |champion_id: String| {
                                                let mut enemy_players_vec = (*enemy_players).clone();
                                                enemy_players_vec[player_index] = EnemyPlayersX {
                                                    champion_id,
                                                    ..enemy_players_vec[player_index].clone()
                                                };
                                                enemy_players.set(enemy_players_vec);
                                            })
                                        })}
                                    />
                                    <Selector<usize>
                                        source_map={all_items}
                                        uri={format!("{}/cdn/items", BACKEND_URL)}
                                        title={"Items"}
                                        selection={SelectionMode::Multiple({
                                            let enemy_players = enemy_players.clone();
                                            let player_index = player_index;
                                            Callback::from(move |items: Vec<usize>| {
                                                let mut enemy_players_vec = (*enemy_players).clone();
                                                enemy_players_vec[player_index] = EnemyPlayersX {
                                                    items,
                                                    ..enemy_players_vec[player_index].clone()
                                                };
                                                enemy_players.set(enemy_players_vec);
                                            })
                                        })}
                                    />
                                </div>
                                <div class={"grid grid-cols-[auto_auto_1fr] items-center gap-2 py-2"}>
                                {
                                    [
                                        (
                                            StatsValue::Level(player.level.to_string()),
                                            format!("{}/cdn/stats/Level.png", BACKEND_URL)
                                        ),
                                        (
                                            StatsValue::MaxHealth(player.stats.health.to_string()),
                                            format!("{}/cdn/stats/Health.png", BACKEND_URL)
                                        ),
                                        (
                                            StatsValue::Armor(player.stats.armor.to_string()),
                                            format!("{}/cdn/stats/Armor.png", BACKEND_URL)
                                        ),
                                        (
                                            StatsValue::MagicResist(player.stats.magic_resist.to_string()),
                                            format!("{}/cdn/stats/MagicResist.png", BACKEND_URL)
                                        ),
                                    ].into_iter().map(|(label_enum, image_url)| {
                                        let oninput = {
                                            let state_handler = enemy_players.clone();
                                            let label_enum = label_enum.clone();

                                            Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                let mut current_state = (*state_handler).clone();

                                                if let Some(input_value) = input.value().parse::<f64>().ok() {
                                                    match label_enum {
                                                        StatsValue::Level(_) => current_state[player_index].level = input_value as usize,
                                                        StatsValue::MaxHealth(_) => current_state[player_index].stats.health = input_value,
                                                        StatsValue::Armor(_) => current_state[player_index].stats.armor = input_value,
                                                        StatsValue::MagicResist(_) => current_state[player_index].stats.magic_resist = input_value,
                                                        _ => {},
                                                    }
                                                }
                                                state_handler.set(current_state);
                                            })
                                        };

                                        html! {
                                            stat_selector(
                                                image_url,
                                                label_enum,
                                                &oninput
                                            )
                                        }
                                    }).collect::<Html>()
                                }
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
