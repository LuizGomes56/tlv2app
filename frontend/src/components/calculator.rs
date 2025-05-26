use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AbilityLevelSelectorProps {
    pub state_handler: UseStateHandle<usize>,
    pub image_url: String,
}

#[function_component]
fn AbilityLevelSelector(props: &AbilityLevelSelectorProps) -> Html {
    html! {
        <div class="flex items-center gap-2">
            <img
                class="h-8 w-8 aspect-square"
                src={props.image_url.clone()}
                alt="Ability"
            />
            <input
                class="bg-slate-800 w-16 h-8 rounded-md text-center"
                type="number"
                value={(*props.state_handler).to_string()}
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
    pub label: &'static str,
    pub state_handler: UseStateHandle<usize>,
}

#[function_component]
fn StatSelector(props: &StatSelectorProps) -> Html {
    html! {
        <div class="flex items-center gap-2">
            <img
                class="h-4 w-4 aspect-square"
                src={props.image_url.clone()}
                alt="Ability"
            />
            <span>{props.label}</span>
            <input
                class="rounded-md bg-slate-800 w-16 h-6 text-center"
                type="number"
                min="0"
                max="999999"
                aria-label="Ability"
            />
        </div>
    }
}

#[function_component]
pub fn Calculator() -> Html {
    let active_player_champion_id = use_state(|| String::from("Neeko"));
    let active_player_q_level = use_state(|| 5usize);
    let active_player_w_level = use_state(|| 5usize);
    let active_player_e_level = use_state(|| 2usize);
    let active_player_r_level = use_state(|| 2usize);
    let active_player_items = use_state(|| Vec::<usize>::from([4645, 3089]));
    let active_player_runes = use_state(|| Vec::<usize>::new());
    let active_player_level = use_state(|| 18usize);

    let enemy_player_champion_id = use_state(|| String::from("Gwen"));
    let enemy_player_items = use_state(|| Vec::<usize>::from([4645, 3089]));
    let enemy_player_level = use_state(|| 14usize);

    html! {
        <div class="flex justify-center ">
            <div class="flex flex-col gap-4 max-h-screen overflow-y-auto">
                <div class="flex relative">
                    <img
                        class="h-32 img-clipped"
                        src={format!("img/centered/{}_0.jpg", *active_player_champion_id.clone())}
                        alt="Banner"
                    />
                </div>
                <div class="flex flex-col gap-4 p-4 bg-slate-900">
                    <div>{ "Champions" }</div>
                    <div>{ "Items" }</div>
                    <div>{ "Runes" }</div>
                </div>
                <section class="flex flex-col px-12 gap-1 relative">
                    {["Q", "W", "E", "R"].into_iter().map(|ability| {
                        let image_url = format!(
                            "img/abilities/{}{}.png",
                            *active_player_champion_id.clone(),
                            ability
                        );

                        html! {
                            <AbilityLevelSelector
                                image_url={image_url}
                                state_handler={match ability {
                                    "Q" => active_player_q_level.clone(),
                                    "W" => active_player_w_level.clone(),
                                    "E" => active_player_e_level.clone(),
                                    "R" => active_player_r_level.clone(),
                                    _ => active_player_q_level.clone(),
                                }}
                            />
                        }
                    }).collect::<Html>()}
                </section>
                <div>
                    {[
                        ("Level", "img/stats/level.png", active_player_level)
                    ].into_iter().map(|(label, image_url, state_handler)| {
                        html! {
                            <StatSelector
                                image_url={image_url}
                                label={label}
                                state_handler={state_handler}
                            />
                        }
                    }).collect::<Html>()}
                </div>
            </div>
            <span>{ "..." }</span>
        </div>
    }
}
