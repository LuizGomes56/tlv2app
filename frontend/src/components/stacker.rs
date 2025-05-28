use std::collections::HashMap;

use uuid::Uuid;
use yew::prelude::*;

use crate::{
    IMG_CDN,
    components::base_table::create_image,
    model::{realtime::InstanceDamage, traits::EnemyLike},
};

#[derive(PartialEq, Properties)]
pub struct MakeStackerHeaderProps {
    pub urls: Vec<String>,
}

#[function_component(MakeStackerHeader)]
pub fn make_stacker_header(props: &MakeStackerHeaderProps) -> Html {
    props
        .urls
        .iter()
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

#[derive(PartialEq, Properties)]
pub struct StackerProps<T>
where
    T: EnemyLike + PartialEq,
{
    pub stack: Vec<StackInstance>,
    pub enemies: Vec<T>,
}

#[function_component(Stacker)]
pub fn stacker<T: EnemyLike + PartialEq>(props: &StackerProps<T>) -> Html {
    html! {
        <table>
            <thead>
                <tr>
                    <th>
                        <span>{ "Name" }</span>
                    </th>
                    <MakeStackerHeader urls={vec![
                        format!("{}/other/sigma.svg", IMG_CDN),
                        format!("{}/other/heart.svg", IMG_CDN),
                        format!("{}/other/percent.svg", IMG_CDN)
                    ]} />
                </tr>
            </thead>
            <tbody>
                {props.enemies.iter().map(|enemy| {
                    let mut total_damage = 0f64;
                    let damages = enemy.get_damages();
                    let current_stats = enemy.get_current_stats();
                    let enemy_champion_id = enemy.get_champion_id();
                    let enemy_champion_name = enemy.get_champion_name();

                    for instance_value in props.stack.iter() {
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
                                <div class="flex items-center gap-2">
                                    <img
                                        src={format!("{}/champions/{}.png", IMG_CDN, &enemy_champion_id)}
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
    stack: UseStateHandle<Vec<StackInstance>>,
    champion_id: String,
    source: String,
    map: HashMap<String, String>,
) -> Html {
    html! {
        <>
            {
                map.iter().map(|(keyname, _)| {
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
                                        source: source.clone(),
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
                                Some(champion_id.clone()),
                                &source
                            )}
                        </button>
                    }
                }).collect::<Html>()
            }
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct StackSelectorProps {
    pub stack: UseStateHandle<Vec<StackInstance>>,
    pub champion_id: String,
    pub instances: (
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, String>,
    ),
}

#[function_component(StackSelector)]
pub fn stack_selector(props: &StackSelectorProps) -> Html {
    let (abilities, items, runes) = props.instances.clone();

    html! {
        <div class="flex flex-col gap-4">
            <h2 class="text-shadow text-xl font-bold">
                { "Selector" }
            </h2>
            <div class="flex flex-wrap gap-2">
                {make_stack_event(
                    props.stack.clone(),
                    props.champion_id.clone(),
                    "abilities".to_string(),
                    abilities
                )}
                {make_stack_event(
                    props.stack.clone(),
                    props.champion_id.clone(),
                    "items".to_string(),
                    items
                )}
                {make_stack_event(
                    props.stack.clone(),
                    props.champion_id.clone(),
                    "runes".to_string(),
                    runes
                )}
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct StackDropperProps {
    pub stack: UseStateHandle<Vec<StackInstance>>,
    pub champion_id: Option<String>,
}

#[function_component(StackDropper)]
pub fn stack_dropper(props: &StackDropperProps) -> Html {
    let stack_instances = (*props.stack).clone();
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
                        let stack = props.stack.clone();
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
                                props.champion_id.clone(),
                                &instance.source
                            )}
                        </button>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
