use crate::{
    IMG_CDN,
    components::{
        base_table::base_table,
        comparison_header::comparison_header,
        comparison_table::comparison_table,
        stacker::{StackInstance, stack_dropper, stack_selector, stacker},
    },
    model::{
        realtime::{CurrentPlayer, Enemy, Realtime, Scoreboard},
        traits::CurrentPlayerLike,
    },
};
use std::rc::Rc;
use yew::prelude::*;

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
                    src={format!("{}/champions/{}.png", IMG_CDN, score.champion_id.clone())}
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
                            "{}/centered/{}_0.jpg",
                            IMG_CDN,
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
                        <img
                            class="h-4 w-4 aspect-square flex-shrink-0"
                            src={format!("{}/other/copy.svg", IMG_CDN)}
                            alt="Copy"
                        />
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
                    {
                        base_table(
                            current_player,
                            enemies,
                        )
                    }
                </div>
                {
                    props.game_data.compared_items.iter().map(|(item_id, value)| {
                        html! {
                            <div class="shadow-container bg-slate-900">
                                <div class="flex flex-col">
                                    {
                                        comparison_header(
                                            value,
                                            item_id,
                                        )
                                    }
                                    <div class="overflow-auto">
                                        {
                                            comparison_table(
                                                current_player,
                                                enemies,
                                                item_id.clone(),
                                            )
                                        }
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
                <div class="p-4 grid grid-cols-[1fr_auto] gap-4 shadow-container bg-slate-900">
                    <div class="flex flex-col gap-4">
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
                                Some(current_player.champion_id.clone()),
                            )
                        }
                    </div>
                    <div class="overflow-auto">
                        {
                            stacker(
                                &stack,
                                &enemies,
                            )
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
