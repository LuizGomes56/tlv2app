use std::collections::HashMap;

use uuid::Uuid;
use yew::prelude::*;

use crate::{
    BACKEND_URL,
    components::base_table::create_image,
    model::{realtime::InstanceDamage, traits::EnemyLike},
};

pub fn make_stacker_header(urls: &[String]) -> Html {
    urls.iter()
        .map(|url| {
            html! {
                <th>
                    <img
                        src={url.clone()}
                        alt="StackerTable"
                    />
                </th>
            }
        })
        .collect::<Html>()
}

#[derive(PartialEq, Clone)]
pub struct StackInstance {
    id: String,
    pub keyname: String,
    pub source: String,
    pub is_maximum: bool,
}

pub fn stacker<T: EnemyLike>(stack: &Vec<StackInstance>, enemies: &Vec<T>) -> Html {
    html! {
        <table>
            <thead>
                <tr>
                    <th>
                        <span>{ "Name" }</span>
                    </th>
                    {
                        make_stacker_header(&[
                            format!("{}/cdn/other/sigma.svg", BACKEND_URL),
                            format!("{}/cdn/other/heart.svg", BACKEND_URL),
                            format!("{}/cdn/other/percent.svg", BACKEND_URL)
                        ])
                    }
                </tr>
            </thead>
            <tbody>
                {enemies.iter().map(|enemy| {
                    let mut total_damage = 0f64;
                    let damages = enemy.get_damages();
                    let current_stats = enemy.get_current_stats();
                    let enemy_champion_id = enemy.get_champion_id();
                    let enemy_champion_name = enemy.get_champion_name();

                    for instance_value in stack.iter() {
                        let mut accumulator = |damagelike: &Option<&InstanceDamage>| {
                            if let Some(instance_damage) = damagelike {
                                if instance_value.is_maximum {
                                    total_damage += instance_damage.maximum_damage;
                                }
                                else {
                                    total_damage += instance_damage.minimum_damage;
                                }
                            }
                        };
                        match instance_value.source.as_str() {
                            "abilities" => accumulator(&damages.abilities.get(&instance_value.keyname)),
                            "items" => accumulator(&damages.items.get(&instance_value.keyname)),
                            "runes" => accumulator(&damages.runes.get(&instance_value.keyname)),
                            _ => {}
                        }
                    }

                    let final_health = current_stats.health - total_damage;
                    let final_health_percent = final_health / current_stats.health;

                    html! {
                        <tr>
                            <td>
                                <div class="flex items-center justify-start gap-2">
                                    <img
                                        src={format!("{}/cdn/champions/{}.png", BACKEND_URL, &enemy_champion_id)}
                                        alt="Champion"
                                    />
                                    <span class="text-sm max-w-24 truncate">
                                        { enemy_champion_name.clone() }
                                    </span>
                                </div>
                            </td>
                            <td>
                                <span>
                                    { format!("{:.0}", total_damage) }
                                </span>
                            </td>
                            <td>
                                <span>
                                    { format!("{:.0}", final_health) }
                                </span>
                            </td>
                            <td>
                                <span>
                                    { format!("{:.2}%", (1f64 - final_health_percent) * 100f64) }
                                </span>
                            </td>
                        </tr>
                    }
                }).collect::<Html>()}
            </tbody>
        </table>
    }
}

fn make_stack_event(
    stack: &UseStateHandle<Vec<StackInstance>>,
    champion_id: &str,
    source: String,
    map: HashMap<String, String>,
) -> Html {
    let mut sorted_map = map.into_iter().collect::<Vec<_>>();
    sorted_map.sort_by(|a, b| a.0.cmp(&b.0));
    html! {
        <>
            {
                sorted_map.iter().map(|(keyname, _)| {
                    let onclick = {
                        let stack = stack.clone();
                        let keyname = keyname.clone();
                        let source = source.clone();

                        Callback::from(move |_| {
                            stack.set({
                                let mut stack_vec = (*stack).clone();
                                stack_vec.push(
                                    StackInstance {
                                        id: Uuid::new_v4().to_string(),
                                        keyname: keyname.clone(),
                                        source: source.clone().to_string(),
                                        is_maximum: false,
                                    },
                                );
                                stack_vec
                            });
                        })
                    };
                    html! {
                        <button {onclick} class="cursor-pointer text-white relative w-8 h-8 flex items-center justify-center">
                            {create_image(
                                keyname,
                                Some(champion_id.to_string()),
                                &source
                            )}
                        </button>
                    }
                }).collect::<Html>()
            }
        </>
    }
}

pub fn stack_selector(
    stack: &UseStateHandle<Vec<StackInstance>>,
    champion_id: String,
    instances: (
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, String>,
    ),
) -> Html {
    let (abilities, items, runes) = instances;

    html! {
        <div class="flex flex-col gap-4">
            <h2 class="text-shadow text-xl font-bold">
                { "Selector" }
            </h2>
            <div class="flex flex-wrap gap-2">
                {make_stack_event(
                    &stack,
                    &champion_id,
                    "abilities".to_string(),
                    abilities
                )}
                {make_stack_event(
                    &stack,
                    &champion_id,
                    "items".to_string(),
                    items
                )}
                {make_stack_event(
                    &stack,
                    &champion_id,
                    "runes".to_string(),
                    runes
                )}
            </div>
        </div>
    }
}

pub fn stack_dropper(
    stack: &UseStateHandle<Vec<StackInstance>>,
    champion_id: Option<String>,
) -> Html {
    let stack_instances = (*stack).clone();
    html! {
        <div class="flex flex-col gap-4">
            {
                if stack_instances.len() > 0 {
                    html!{
                        <h2 class="text-shadow text-xl font-bold">
                            { "Stack" }
                        </h2>
                    }
                } else {
                    html!{}
                }
            }
            <div class="flex flex-wrap gap-2 max-h-40 overflow-y-auto">
                {stack_instances.iter().map(|instance| {
                    let onclick = {
                        let stack = stack.clone();
                        let instance = instance.clone();
                        Callback::from(move |_| {
                            stack.set({
                                let mut stack_vec = (*stack).clone();
                                if let Some(index) = stack_vec.iter().position(|element| element.id == instance.id) {
                                    stack_vec.remove(index);
                                }
                                stack_vec
                            });
                        })
                    };
                    html! {
                        <button
                            class="relative w-8 h-8 flex items-center cursor-pointer justify-center"
                            {onclick}
                        >
                            {create_image(
                                &instance.keyname,
                                champion_id.clone(),
                                &instance.source
                            )}
                        </button>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
