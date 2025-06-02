use crate::{
    IMG_CDN, MAX_FAILURES,
    components::{
        base_table::base_table,
        comparison_header::comparison_header,
        comparison_table::comparison_table,
        stacker::{StackInstance, stack_dropper, stack_selector, stacker},
    },
    img::header::{about_svg, github_svg, play_svg},
    model::{
        realtime::{CurrentPlayer, Enemy, Realtime, Scoreboard},
        traits::CurrentPlayerLike,
    },
    tauriapp::invokers::get_realtime_game,
};
use gloo::timers::callback::Interval;
use std::{ops::Deref, rc::Rc};
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;

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

#[derive(PartialEq, Properties)]
pub struct RealtimeDisplayProps {
    pub game_code: String,
}

#[function_component(RealtimeDisplay)]
pub fn realtime_display(props: &RealtimeDisplayProps) -> Html {
    let game_data = use_state(|| Option::<Rc<Realtime>>::None);
    let hide_champion_state = use_state(|| Vec::<usize>::new());
    let stack = use_state(|| Vec::<StackInstance>::new());
    let interval_state = use_state(|| Option::<Interval>::None);
    let failure_counter = use_mut_ref(|| 0usize);

    let start_game = {
        let interval_state = interval_state.clone();
        let failure_counter = failure_counter.clone();
        let game_data = game_data.clone();

        *failure_counter.borrow_mut() = 0;

        Callback::from(move |_| {
            if interval_state.is_none() {
                let failure_counter = Rc::clone(&failure_counter);
                let cloned_interval_state = interval_state.clone();
                let game_data = game_data.clone();

                let interval = Interval::new(1000, move || {
                    get_realtime_game(game_data.clone(), failure_counter.clone());

                    if *failure_counter.borrow() >= (MAX_FAILURES - 1) {
                        web_sys::console::log_1(&"Parando após 10 falhas".into());
                        cloned_interval_state.set(None);
                    }
                });

                interval_state.set(Some(interval));
            }
        })
    };

    let stop_game = {
        let interval_state = interval_state.clone();

        Callback::from(move |_: MouseEvent| {
            interval_state.set(None);
        })
    };

    // <button class="cursor-pointer" onclick={stop_game}>
    //     { "Stop Game" }
    // </button>

    if let Some(game_data) = game_data.deref().clone() {
        let current_player = &game_data.current_player;
        let enemies = &game_data.enemies;
        let game_time = game_data.game_information.game_time;
        let game_time_minutes = game_time as i32 / 60;
        let game_time_seconds = game_time as i32 % 60;

        let mut ally_scoreboard = Vec::<Scoreboard>::new();
        let mut enemy_scoreboard = Vec::<Scoreboard>::new();

        for player_score in &game_data.scoreboard {
            if player_score.team == current_player.team {
                ally_scoreboard.push(player_score.clone());
            } else {
                enemy_scoreboard.push(player_score.clone());
            }
        }

        html! {
            <div class="flex gap-4 p-4 w-full container mx-auto">
                <div class="flex flex-col gap-4 max-w-md">
                    <div class="flex flex-col shadow-container bg-custom-900">
                        <img
                            class="img-clipped h-32"
                            src={format!(
                                "{}/centered/{}_0.jpg",
                                IMG_CDN,
                                current_player.champion_id
                            )}
                            alt="Champion"
                        />
                        <div class="flex justify-between font-bold bg-custom-900 items-center text-slate-300 p-4 gap-8">
                            <span class="text-shadow truncate min-w-0">{format!("{} - {}", current_player.riot_id, current_player.champion_name)}</span>
                            <span class="text-shadow text-nowrap">{format!("{}m {:02}s", game_time_minutes, game_time_seconds)}</span>
                        </div>
                    </div>
                    <div class="flex flex-col shadow-container bg-custom-900">
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
                                        let hide_champion_hook_clone = hide_champion_state.clone();
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
                                                if hide_champion_state.contains(&index) {
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
                    <div class="shadow-container bg-custom-900">
                        {
                            base_table(
                                current_player,
                                enemies,
                            )
                        }
                    </div>
                    {
                        game_data.compared_items.iter().map(|(item_id, value)| {
                            html! {
                                <div class="shadow-container bg-custom-900">
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
                    <div class="p-4 grid grid-cols-[1fr_auto] gap-4 shadow-container bg-custom-900">
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
    } else {
        html! {
            <div class="flex flex-col gap-12 max-h-screen overflow-y-auto p-12">
                <div class="flex items-center gap-8">
                    <div class="relative">
                        <h1 class="font-bold text-4xl bg-gradient-to-r from-indigo-300 via-purple-400 to-pink-400 bg-clip-text text-transparent mb-2">{ "Realtime Mode" }</h1>
                        <div class="w-24 h-1 bg-gradient-to-r from-indigo-500 to-purple-500 rounded-full"></div>
                    </div>
                    <button
                        class="bg-gradient-to-r from-indigo-800 via-purple-800 to-pink-800 text-nowrap place-self-center flex items-center justify-center gap-3 cursor-pointer py-2.5 px-5 text-lg font-semibold transition rounded-lg"
                        onclick={start_game}
                    >
                        <div class="w-4 h-4">{ play_svg() }</div>
                        <span>{ "Load my current game data" }</span>
                    </button>
                </div>
                <div class="grid grid-cols-2 gap-10">
                    <div class="w-full flex flex-col gap-10">
                        <div class="w-full flex flex-col gap-3 leading-8">
                            <h3 class="flex text-lg font-semibold items-center gap-3 mb-3 text-white">
                                { "How to use it?" }
                            </h3>
                            <p class="text-slate-300">
                                { "In order for this feature to work " }
                                <span class="font-semibold text-green-300 bg-green-900/30 px-2 py-1 rounded-md border border-green-700/50">{ "you must have left champion selection phase" }</span>
                                { " and be playing in a valid gamemode such as "}
                                <span class="font-semibold text-rose-300 bg-rose-900/30 px-2 py-1 rounded-md border border-rose-700/50">{ "Summoner's Rift or ARAM" }</span>
                            </p>
                            <p class="text-slate-300 leading-7">
                                { "It is possible to look at your friend's game data instead. In order to do this,  " }
                                <span class="font-semibold text-indigo-300 bg-indigo-900/30 px-2 py-1 rounded-md border border-indigo-700/50">{ "they must be running this app during their game" }</span>
                                { " and follow the same steps as mentioned." }
                            </p>
                        </div>
                        <div class="leading-7">
                            <h3 class="flex text-lg font-semibold items-center gap-3 mb-3 text-white">
                                { "Troubleshooting Tips" }
                            </h3>
                            <div class="flex flex-col gap-2 text-slate-300">
                                <div class="flex gap-3">
                                    <span class="text-white font-bold">{ "1." }</span>
                                    <p>
                                        { "The champion you're playing might be unsupported or recently broken. If you are a developer, you can check the " }
                                        <span class="font-semibold text-white">{ "Formulas" }</span>
                                        { " section and inspect the " }
                                        <code class="font-semibold text-yellow-300 bg-yellow-900/30 px-2 py-1 rounded-md border border-yellow-300/50">{ "json" }</code>{ " and " }
                                        <code class="font-semibold text-orange-300 bg-orange-900/30 px-2 py-1 rounded-md border border-orange-300/50">{ "rust" }</code>{ " code, or open an issue on " }
                                        <a
                                            class="transition-colors w-fit font-semibold text-purple-300 hover:text-violet-400 hover:bg-violet-900/30 hover:border-violet-700/50 bg-purple-900/30 px-2 py-1 rounded-md border border-purple-700/50"
                                            href="https://github.com/LuizGomes56/tutorlolv2/tree/master/src"
                                            target="_blank"
                                        >
                                            { "Github" }
                                        </a>
                                    </p>
                                </div>
                                <div class="flex gap-3">
                                    <span class="text-white font-bold">{ "2." }</span>
                                    <p>{ "My calculator service may be temporarily down." }</p>
                                </div>
                                <div class="flex gap-3">
                                    <span class="text-white font-bold">{ "3." }</span>
                                    <p>{ "Riot may have changed their API unexpectedly." }</p>
                                </div>
                                <div class="flex gap-3">
                                    <span class="text-white font-bold">{ "4." }</span>
                                    <p>{ "Your game code could have been invalidated." }</p>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="flex flex-col gap-4">
                        <h3 class="flex text-lg font-semibold items-center gap-3 mb-3 text-white">
                            { "Source code and formulas" }
                        </h3>
                        <p class="text-slate-300">
                            { "Formulas used here are partially available in the " }
                            <span class="font-semibold text-white">{ "Formulas" }</span>
                            { " section. You can explore how it all works by visiting my Github repositories listed below. More details about the project can be found in other sections of the application." }
                        </p>
                        <div class="grid sm:grid-cols-2 gap-3">
                            <a class="group bg-zinc-950/60 text-blue-400 hover:text-sky-400 hover:bg-zinc-800/60 border border-zinc-700 hover:border-zinc-600 rounded-lg p-4 transition-all duration-200"
                            href="https://github.com/LuizGomes56/tutorlolv2" target="_blank">
                                <div class="flex items-center gap-3">
                                    <div class="w-5 h-5 flex-shrink-0">
                                        { github_svg() }
                                    </div>
                                    <span class="font-semibold">{ "TutorLoLv2 Server" }</span>
                                </div>
                            </a>
                            <a class="group bg-zinc-950/60 text-blue-400 hover:text-sky-400 hover:bg-zinc-800/60 border border-zinc-700 hover:border-zinc-600 rounded-lg p-4 transition-all duration-200"
                            href="https://github.com/LuizGomes56/tlv2app" target="_blank">
                                <div class="flex items-center gap-3">
                                    <div class="w-5 h-5 flex-shrink-0">
                                        { github_svg() }
                                    </div>
                                    <span class="font-semibold">{ "TutorLoLv2 Windows/WASM" }</span>
                                </div>
                            </a>
                        </div>
                        <p class="text-slate-300">
                            { "All the data is retrieved directly from the following link:" }
                        </p>
                        <a
                            class="transition-colors w-fit font-semibold text-blue-300 hover:text-sky-400 hover:bg-sky-900/30 hover:border-sky-700/50 bg-blue-900/30 px-2 py-1 rounded-md border border-blue-700/50"
                            href="https://127.0.0.1:2999/liveclientdata/allgamedata"
                            target="_blank"
                        >
                            { "https://127.0.0.1:2999/liveclientdata/allgamedata" }
                        </a>
                        <div class="flex items-center gap-2 text-[#8E8F93]">
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M2.166 4.999A11.954 11.954 0 0010 1.944 11.954 11.954 0 0017.834 5c.11.65.166 1.32.166 2.001 0 5.225-3.34 9.67-8 11.317C5.34 16.67 2 12.225 2 7c0-.682.057-1.35.166-2.001zm11.541 3.708a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                            </svg>
                            <span class="font-semibold">{ "No unauthorized data sources are used." }</span>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
