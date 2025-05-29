use std::{collections::HashMap, ops::Deref, rc::Rc};

use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    IMG_CDN, apply_stat,
    components::{
        base_table::BaseTable,
        comparison_header::ComparisonHeader,
        comparison_table::ComparisonTable,
        selector::{SelectionMode, Selector},
        stacker::{StackDropper, StackInstance, StackSelector, Stacker},
        value_cell::ValueCell,
    },
    model::{
        calculator::{ActivePlayerX, Calculator, CurrentPlayerX, EnemyPlayersX, EnemyX, GameX},
        traits::CurrentPlayerLike,
    },
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
struct ServerResponse<T> {
    data: T,
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
    let all_champions = use_state(|| Rc::<HashMap<String, String>>::new(HashMap::new()));
    let all_items = use_state(|| Rc::<HashMap<usize, String>>::new(HashMap::new()));
    let all_runes = use_state(|| Rc::<HashMap<usize, String>>::new(HashMap::new()));

    let client = Client::new();

    let error_occurred = use_state(|| false);

    let onerror_callback = {
        let error_occurred = error_occurred.clone();
        Callback::from(move |_| {
            error_occurred.set(true);
        })
    };

    {
        let client = client.clone();
        let all_champions = all_champions.clone();
        let all_items = all_items.clone();
        let all_runes = all_runes.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let champion_response = client
                    .get("http://localhost:8082/api/static/champions")
                    .send()
                    .await
                    .unwrap();

                let items_response = client
                    .get("http://localhost:8082/api/static/items")
                    .send()
                    .await
                    .unwrap();

                let runes_response = client
                    .get("http://localhost:8082/api/static/runes")
                    .send()
                    .await
                    .unwrap();

                let champion_data = champion_response
                    .json::<ServerResponse<HashMap<String, String>>>()
                    .await;

                let items_data = items_response
                    .json::<ServerResponse<HashMap<usize, String>>>()
                    .await;

                let runes_data = runes_response
                    .json::<ServerResponse<HashMap<usize, String>>>()
                    .await;

                if let Some(ServerResponse { data }) = champion_data.ok() {
                    all_champions.set(Rc::new(data));
                }
                if let Some(ServerResponse { data }) = items_data.ok() {
                    all_items.set(Rc::new(data));
                }
                if let Some(ServerResponse { data }) = runes_data.ok() {
                    all_runes.set(Rc::new(data));
                }
            });
            || ()
        });
    }

    {
        let client = client.clone();
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
                };

                spawn_local(async move {
                    let json_value = json!({
                        "game": game_state,
                        "simulated_items": simulated_items.deref().clone()
                    });

                    let res = client
                        .post("http://localhost:8082/api/games/calculator")
                        .json(&json_value)
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
        <div class="flex flex-wrap gap-4 p-4 justify-center">
            <div class="flex flex-col">
                <div class="flex relative">
                    <img
                        class="h-32 img-clipped"
                        src={if *error_occurred {
                            format!("{}/splash/{}_0.jpg", IMG_CDN, active_player.champion_id)
                        } else {
                            format!("{}/centered/{}_0.jpg", IMG_CDN, active_player.champion_id)
                        }}
                        onerror={onerror_callback}
                        alt="Banner"
                    />
                </div>
                <div class="flex flex-col bg-slate-900">
                    <Selector<String>
                        source_map={all_champions.deref().clone()}
                        uri={format!("{}/champions", IMG_CDN)}
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
                        source_map={all_items.deref().clone()}
                        uri={format!("{}/items", IMG_CDN)}
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
                        source_map={all_runes.deref().clone()}
                        uri={format!("{}/runes", IMG_CDN)}
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
                <section class="grid grid-cols-4 gap-2 py-2">
                    {
                        ["Q", "W", "E", "R"].into_iter().map(|ability| {
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
                        }).collect::<Html>()
                    }
                    <ValueCell
                        image_source={"EarthDragon.png"}
                        value={ally_earth_dragons.deref().clone().to_string()}
                        oninput={
                            let ally_earth_dragons = ally_earth_dragons.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                ally_earth_dragons.set(input.value().parse().unwrap_or_default());
                            })
                        }
                    />
                    <ValueCell
                        image_source={"FireDragon.png"}
                        oninput={{
                            let ally_fire_dragons = ally_fire_dragons.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                ally_fire_dragons.set(input.value().parse().unwrap_or_default());
                            })
                        }}
                        value={ally_fire_dragons.deref().clone().to_string()}
                    />
                    <ValueCell
                        image_source={"stack.svg"}
                        oninput={{
                            let active_player_stacks = active_player_stacks.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                active_player_stacks.set(input.value().parse().unwrap_or_default());
                            })
                        }}
                        value={active_player_stacks.deref().clone().to_string()}
                    />
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
                            StatsValue::CurrentHealth(active_player.champion_stats.current_health.to_string()),
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
                        <div class="flex flex-col gap-4 flex-1">
                            <div class="overflow-auto">
                                <BaseTable<CurrentPlayerX, EnemyX>
                                    current_player={current_player.clone()}
                                    enemies={enemies.clone()}
                                />
                            </div>
                            {
                                calculator_data.compared_items.iter().map(|(item_id, value)| {
                                    html! {
                                        <div class="shadow-container bg-slate-900">
                                            <div class="flex flex-col">
                                                <ComparisonHeader
                                                    value={value.clone()}
                                                    item_id={item_id.to_string().clone()}
                                                />
                                                <div class="overflow-auto">
                                                    <ComparisonTable<CurrentPlayerX, EnemyX>
                                                        current_player={current_player.clone()}
                                                        enemies={enemies.clone()}
                                                        item_id={calculator_data.best_item.clone().to_string()}
                                                    />
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                            <div class="p-4 grid grid-cols-[1fr_auto] gap-4 shadow-container bg-slate-900">
                                <div class="flex flex-col gap-4">
                                    <StackSelector
                                        stack={stack.clone()}
                                        champion_id={current_player.champion_id.clone()}
                                        instances={current_player.get_damaging_instances()}
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
            <div class="flex flex-col">
                <div class="grid grid-cols-[auto_1fr_1fr_auto] mb-4 gap-2 h-12 text-center text-slate-400 bg-slate-800">
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
                        class="flex items-center justify-center aspect-square select-none cursor-pointer flex-shrink-0 font-bold">
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
                                    class="select-none cursor-pointer">
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
                                    class="select-none cursor-pointer">
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
                        class="flex items-center justify-center aspect-square select-none cursor-pointer flex-shrink-0 font-bold">
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
                                        class="h-32 img-clipped"
                                        src={format!("{}/centered/{}_0.jpg", IMG_CDN, player.champion_id)}
                                        alt="Banner"
                                    />
                                </div>
                                <div class="flex flex-col bg-slate-900">
                                    <Selector<String>
                                        source_map={all_champions.deref().clone()}
                                        uri={format!("{}/champions", IMG_CDN)}
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
                                        source_map={all_items.deref().clone()}
                                        uri={format!("{}/items", IMG_CDN)}
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
                                <div class="grid grid-cols-[auto_auto_1fr] items-center gap-2 py-2">
                                {
                                    [
                                        (
                                            StatsValue::Level(player.level.to_string()),
                                            format!("{}/stats/Level.png", IMG_CDN)
                                        ),
                                        (
                                            StatsValue::MaxHealth(player.stats.health.to_string()),
                                            format!("{}/stats/Health.png", IMG_CDN)
                                        ),
                                        (
                                            StatsValue::Armor(player.stats.armor.to_string()),
                                            format!("{}/stats/Armor.png", IMG_CDN)
                                        ),
                                        (
                                            StatsValue::MagicResist(player.stats.magic_resist.to_string()),
                                            format!("{}/stats/MagicResist.png", IMG_CDN)
                                        ),
                                    ].into_iter().map(|(label_enum, image_url)| {
                                        let cloned_enum = label_enum.clone();
                                        let (name, value) = label_enum.get_labels();

                                        let oninput = {
                                            let state_handler = enemy_players.clone();

                                            Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                let mut current_state = (*state_handler).clone();

                                                if let Some(input_value) = input.value().parse::<f64>().ok() {
                                                    match cloned_enum {
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
                                                    src={image_url.clone()}
                                                    alt="Ability"
                                                />
                                                <span class="text-sm text-shadow">{name}</span>
                                            </>
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
