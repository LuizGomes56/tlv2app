use crate::{
    BACKEND_URL, MAX_FAILURES,
    components::{
        base_table::base_table,
        comparison_header::comparison_header,
        comparison_table::comparison_table,
        stacker::{StackInstance, stack_dropper, stack_selector, stacker},
    },
    externcalls::invokers::get_realtime_game,
    img::icons::{about_svg, github_svg, play_svg},
    model::{
        realtime::{CurrentPlayer, Enemy, Realtime, Scoreboard},
        traits::CurrentPlayerLike,
    },
};
use gloo::timers::callback::Interval;
use std::{cell::RefCell, ops::Deref, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, console, window};
use yew::prelude::*;

fn make_scoreboard(score: &Scoreboard) -> Html {
    html! {
        <section class="grid grid-cols-[1fr_auto] p-2 gap-4 items-center">
            <div class="grid grid-cols-[auto_1fr] items-center gap-1.5">
                <img
                    class="min-w-8 h-8 aspect-square flex-shrink-0"
                    src={format!("{}/cdn/champions/{}.png", BACKEND_URL, score.champion_id.clone().unwrap_or(score.champion_name.clone()))}
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

fn fetch_game(
    interval_state: &UseStateHandle<Option<Interval>>,
    failure_counter: &Rc<RefCell<usize>>,
    game_data: &UseStateHandle<Option<Rc<Realtime>>>,
    game_code: &String,
) {
    let failure_counter = Rc::clone(&failure_counter);
    let cloned_interval_state = interval_state.clone();
    let game_data = game_data.clone();
    let game_code = game_code.clone();

    let interval = Interval::new(1000, move || {
        get_realtime_game(
            game_code.clone(),
            game_data.clone(),
            failure_counter.clone(),
        );

        if *failure_counter.borrow() >= (MAX_FAILURES - 1) {
            web_sys::console::log_1(&"Parando após 10 falhas".into());
            cloned_interval_state.set(None);
        }
    });

    interval_state.set(Some(interval));
}

#[derive(PartialEq, Properties)]
pub struct RealtimeDisplayProps {
    pub game_code_state: UseStateHandle<String>,
}

#[function_component(RealtimeDisplay)]
pub fn realtime_display(props: &RealtimeDisplayProps) -> Html {
    let game_data = use_state(|| Option::<Rc<Realtime>>::None);
    let hide_champion_state = use_state(|| Vec::<usize>::new());
    let stack = use_state(|| Vec::<StackInstance>::new());
    let interval_state = use_state(|| Option::<Interval>::None);
    let failure_counter = use_mut_ref(|| 0usize);
    let error_occurred = use_state(|| false);
    let game_code = props.game_code_state.deref().clone();

    let start_game = {
        let interval_state = interval_state.clone();
        let failure_counter = failure_counter.clone();
        let game_data = game_data.clone();
        let game_code = game_code.clone();

        *failure_counter.borrow_mut() = 0;

        Callback::from(move |_: MouseEvent| {
            if interval_state.is_none() && game_code.len() == 6 {
                fetch_game(&interval_state, &failure_counter, &game_data, &game_code);
            }
        })
    };

    let stop_game = {
        let interval_state = interval_state.clone();

        Callback::from(move |_: MouseEvent| {
            web_sys::console::log_1(&"Code changed. Cancelling game requests".into());
            interval_state.set(None);
        })
    };

    let change_game_code = {
        let game_code = props.game_code_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            game_code.set(input.value());
        })
    };

    {
        let game_data = game_data.clone();
        let game_code = game_code.clone();
        use_effect_with(props.game_code_state.clone(), move |_| {
            interval_state.set(None);
            if interval_state.is_none() && game_code.len() == 6 {
                fetch_game(&interval_state, &failure_counter, &game_data, &game_code);
            }
        })
    }

    let onerror_callback = {
        let error_occurred = error_occurred.clone();
        Callback::from(move |_| {
            error_occurred.set(true);
        })
    };

    if let Some(game_data) = game_data.deref().clone() {
        let current_player = &game_data.current_player;
        let enemies = &game_data
            .enemies
            .iter()
            .enumerate()
            .filter(|(idx, _)| !hide_champion_state.deref().contains(idx))
            .map(|(_, enm)| enm.clone())
            .collect::<Vec<_>>();
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
            <div class="flex gap-4 p-4 w-full max-h-screen overflow-y-auto">
                <div class="flex flex-col gap-4 max-w-md">
                    <div class="flex flex-col shadow-container bg-custom-900">
                        <img
                            class="img-clipped h-32"
                            src={if *error_occurred {
                                format!("{}/cdn/splash/{}_0.jpg", BACKEND_URL, current_player.champion_id)
                            } else {
                                format!("{}/cdn/centered/{}_0.jpg", BACKEND_URL, current_player.champion_id)
                            }}
                            onerror={onerror_callback}
                            alt="Champion"
                        />
                        <div class="flex justify-between font-bold bg-custom-900 items-center text-slate-300 p-4 gap-8">
                            <span class="text-shadow truncate min-w-0">{format!("{} - {}", current_player.riot_id, current_player.champion_name)}</span>
                            <span class="text-shadow text-nowrap">{format!("{}m {:02}s", game_time_minutes, game_time_seconds)}</span>
                        </div>
                    </div>
                    <div class="flex flex-col shadow-container bg-custom-900">
                        <div class="grid grid-cols-2">
                            <button
                                class="cursor-pointer flex items-center gap-2 p-4 bg-indigo-950 justify-center"
                                onclick={{
                                    let game_code = game_code.clone();
                                    Callback::from(move |_| {
                                    let game_code = game_code.clone();

                                    spawn_local(async move {
                                        if let Some(window) = window() {
                                            let _ = window.navigator().clipboard().write_text(&game_code);
                                        }
                                    });
                                })}}
                            >
                                <img
                                    class="h-4 w-4 aspect-square flex-shrink-0"
                                    src={format!("{}/cdn/other/copy.svg", BACKEND_URL)}
                                    alt="Copy"
                                />
                                <span class="font-bold text-sm text-shadow">{format!("Game Code - {}", game_code)}</span>
                            </button>
                            <button
                                onclick={stop_game}
                                class="cursor-pointer flex items-center bg-emerald-950 gap-2 p-4 justify-center"
                            >
                                <img
                                    class="h-4 w-4 aspect-square flex-shrink-0"
                                    src={format!("{}/cdn/other/stop.svg", BACKEND_URL)}
                                    alt="Stop"
                                />
                                <span class="font-bold text-sm text-shadow">{ "Stop Game" }</span>
                            </button>
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
                        <h1 class="font-bold text-4xl bg-gradient-to-r from-blue-400 via-cyan-400 to-emerald-400 bg-clip-text text-transparent mb-2">{ "Realtime Mode" }</h1>
                        <div class="w-24 h-1 bg-gradient-to-r from-blue-500 to-cyan-500 rounded-full"></div>
                    </div>
                    <button
                        class="bg-gradient-to-r from-blue-800 via-cyan-800 to-emerald-700 text-nowrap place-self-center flex items-center justify-center gap-3 cursor-pointer py-2.5 px-5 text-lg font-semibold transition rounded-lg"
                        onclick={start_game}
                    >
                        <div class="w-4 h-4">{ play_svg() }</div>
                        <span>{ "Load my current game data" }</span>
                    </button>
                </div>
                <div class="grid grid-cols-2 gap-10 leading-8">
                    <div class="w-full flex flex-col gap-4">
                        <h3 class="flex text-lg font-semibold items-center gap-3 mb-3 text-white">
                            { "How to use it?" }
                        </h3>
                        <p class="text-slate-300">
                            { "In order for this feature to work " }
                            <span class="font-semibold text-green-300 bg-green-900/30 px-2 py-1 rounded-md">{ "you must have left champion selection phase" }</span>
                            { " and be playing in a valid gamemode such as "}
                            <span class="font-semibold text-rose-300 bg-rose-900/30 px-2 py-1 rounded-md">{ "Summoner's Rift or ARAM" }</span>
                        </p>
                        <p class="text-slate-300">
                            { "It is possible to look at your friend's game data instead. To achieve this behavior, " }
                            <span class="font-semibold text-indigo-300 bg-indigo-900/30 px-2 py-1 rounded-md">{ "they must be running this app during their game" }</span>
                            { " and follow the same steps as mentioned. After, you will have to " }
                            <span class="font-semibold text-emerald-300 bg-emerald-900/30 px-2 py-1 rounded-md">{ "enter their game code" }</span>
                            { " in the box below" }
                        </p>
                        <div class="relative flex items-center gap-2 w-fit">
                            <span class="font-semibold text-emerald-300 bg-emerald-900/30 px-2 py-1 rounded-md">{ "Game code value" }</span>
                            <input
                                type="text"
                                placeholder="000000"
                                value={game_code.clone()}
                                oninput={change_game_code}
                                class="w-28 px-4 h-10 rounded-lg text-indigo-300 bg-indigo-900/30 focus:outline-none focus:border-emerald-400 transition-all duration-200 backdrop-blur-sm tracking-wide"
                            />
                            <div class="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
                                <svg class="w-5 h-5 text-indigo-400/70 group-focus-within:text-purple-400 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
                                </svg>
                            </div>
                        </div>
                    </div>
                    <div>
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
                                    <code class="font-semibold text-yellow-300 bg-yellow-900/30 px-2 py-1 rounded-md">{ "json" }</code>{ " and " }
                                    <code class="font-semibold text-orange-300 bg-orange-900/30 px-2 py-1 rounded-md">{ "rust" }</code>{ " code, or open an issue on " }
                                    <a
                                        class="transition-colors w-fit font-semibold text-purple-300 hover:text-violet-400 hover:bg-violet-900/30 hover:border-violet-700/50 bg-purple-900/30 px-2 py-1 rounded-md"
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
            </div>
        }
    }
}
