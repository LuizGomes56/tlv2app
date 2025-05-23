use crate::{
    components::{
        base_table::BaseTable,
        comparison_table::ComparisonTable,
        stacker::{StackDropper, StackSelector, Stacker},
    },
    model::realtime::{Realtime, Scoreboard},
};
use std::rc::Rc;
use yew::prelude::*;

use super::stacker::StackInstance;

#[derive(PartialEq, Properties)]
pub struct RealtimeDisplayProps {
    pub game_data: Rc<Realtime>,
    pub game_code: String,
}

fn make_scoreboard(score: &Scoreboard) -> Html {
    html! {
        <section class="grid grid-cols-[1fr_auto] p-2 gap-4 items-center">
            <div class="grid grid-cols-[auto_1fr] items-center gap-1.5">
                <img
                    class="min-w-8 h-8 aspect-square flex-shrink-0"
                    src={format!("img/champions/{}.png", score.champion_id.clone())}
                    alt="Champion"
                />
                <div class="flex flex-col leading-none overflow-hidden">
                    <span class="text-shadow truncate text-left text-xs">
                        {score.riot_id.clone()}
                    </span>
                    <span class="text-shadow truncate text-left text-slate-400 text-[10px]">
                        {score.champion_name.clone()}
                    </span>
                </div>
            </div>
            <span class="text-shadow text-sm text-nowrap mr-2">
                {
                    format!("{} / {} / {}",
                    score.kills,
                    score.deaths,
                    score.assists
                )}
            </span>
        </section>
    }
}

#[function_component(RealtimeDisplay)]
pub fn realtime_display(props: &RealtimeDisplayProps) -> Html {
    let current_player = &props.game_data.current_player;
    let enemies = &props.game_data.enemies;
    let game_time = props.game_data.game_information.game_time;
    let game_time_minutes = game_time as i32 / 60;
    let game_time_seconds = game_time as i32 % 60;
    let hide_champion_hook = use_state(|| Vec::<usize>::new());

    let mut ally_scoreboard = Vec::<Scoreboard>::new();
    let mut enemy_scoreboard = Vec::<Scoreboard>::new();

    for player_score in &props.game_data.scoreboard {
        if player_score.team == current_player.team {
            ally_scoreboard.push(player_score.clone());
        } else {
            enemy_scoreboard.push(player_score.clone());
        }
    }

    let stack = use_state(|| Vec::<StackInstance>::new());

    html! {
        <div class="flex gap-4 p-4 w-full container mx-auto">
            <div class="flex flex-col gap-4 max-w-md">
                <div class="flex flex-col shadow-container bg-slate-900">
                    <img
                        class="img-clipped h-32"
                        src={format!(
                            "img/centered/{}_0.jpg",
                            current_player.champion_id
                        )}
                        alt="Champion"
                    />
                    <div class="flex justify-between font-bold bg-slate-900 items-center text-slate-300 p-4 gap-8">
                        <span class="text-shadow truncate min-w-0">{format!("{} - {}", current_player.riot_id, current_player.champion_name)}</span>
                        <span class="text-shadow text-nowrap">{format!("{}m {:02}s", game_time_minutes, game_time_seconds)}</span>
                    </div>
                </div>
                <div class="flex flex-col shadow-container bg-slate-900">
                    <div class="cursor-pointer flex items-center gap-4 p-4 justify-center">
                        <img class="h-4 w-4 aspect-square flex-shrink-0" src="img/other/copy.svg" alt="Copy" />
                        <span class="font-bold text-sm text-shadow">{format!("Game Code - {}", props.game_code)}</span>
                    </div>
                    <div class="grid grid-cols-2">
                        <div class="flex flex-col">
                            <h2 class="text-center text-shadow text-sm py-2 truncate">{ "Your Team" }</h2>
                            <div class="table-like">
                                {ally_scoreboard.iter().map(|score| make_scoreboard(score)).collect::<Html>()}
                            </div>
                        </div>
                        <div class="flex flex-col">
                            <h2 class="text-center text-shadow text-sm py-2 truncate">{ "Show/Hide" }</h2>
                            {
                                enemy_scoreboard.iter().enumerate().map(|(index, score)| {
                                    let hide_champion_hook_clone = hide_champion_hook.clone();
                                    let onclick = Callback::from(move |_: MouseEvent| {
                                        hide_champion_hook_clone.set({
                                            let mut hide_champion_hook = (*hide_champion_hook_clone).clone();
                                            if hide_champion_hook.contains(&index) {
                                                hide_champion_hook.retain(|&i| i != index);
                                            } else {
                                                hide_champion_hook.push(index);
                                            }
                                            hide_champion_hook
                                        });
                                    });
                                    html! {
                                        <button class={format!("cursor-pointer {}",
                                            if hide_champion_hook.contains(&index) {
                                                "table-like-alt-hidden"
                                            } else {
                                                "table-like-alt"
                                            }
                                        )
                                        } {onclick}>
                                            {make_scoreboard(score)}
                                        </button>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                </div>
            </div>
            <div class="flex flex-col gap-4 flex-1">
                <div class="shadow-container bg-slate-900">
                    <BaseTable
                        current_player={current_player.clone()}
                        enemies={enemies.clone()}
                    />
                </div>
                {
                    props.game_data.compared_items.iter().map(|(item_id, value)| {
                        html! {
                            <div class="shadow-container bg-slate-900">
                                <div class="flex flex-col">
                                    <div class="flex flex-col p-3">
                                        <div class="flex justify-between items-center gap-4 pb-3 mb-2.5 border-b border-b border-b-zinc-600 w-full">
                                            <div class="flex items-center gap-4">
                                                <img
                                                    class="w-8 h-8 aspect-square flex-shrink-0"
                                                    src={format!("img/items/{}.png", item_id)}
                                                    alt="Compared Item"
                                                />
                                                <span class="text-shadow font-bold">
                                                    {value.name.clone()}
                                                </span>
                                            </div>
                                            <div class="flex items-center gap-1">
                                                <img
                                                    class="w-4 h-4 aspect-square flex-shrink-0"
                                                    src="img/stats/GoldPer10Seconds.png"
                                                    alt="Gold Cost"
                                                />
                                                <span class="text-yellow-300 text-shadow">{value.gold_cost}</span>
                                            </div>
                                        </div>
                                        <div class="grid grid-cols-3">
                                            {value.prettified_stats.iter().map(|(stat_name, stat_value)| {
                                                let stat_img_path = stat_name
                                                    .to_string()
                                                    .split(' ')
                                                    .collect::<Vec<&str>>()
                                                    .join("");
                                                html! {
                                                    <div class="flex items-center gap-2">
                                                        <img
                                                            class="w-4 h-4 aspect-square flex-shrink-0"
                                                            src={format!("img/stats/{}.png", stat_img_path)}
                                                            alt="Stat"
                                                        />
                                                        <span class="text-sm text-blue-200 text-shadow">
                                                            {format!("{} {}", stat_value.to_string().trim_end_matches(".0"), stat_name)}
                                                        </span>
                                                    </div>
                                                }
                                            }).collect::<Html>()}
                                        </div>
                                    </div>
                                    <div class="overflow-auto">
                                        <ComparisonTable
                                            current_player={current_player.clone()}
                                            enemies={enemies.clone()}
                                            item_id={item_id.clone()}
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
                        <Stacker
                            stack={(*stack).clone()}
                            enemies={enemies.clone()}
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
