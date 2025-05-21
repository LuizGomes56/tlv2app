use crate::model::realtime::{InstanceDamage, Realtime};
use std::{collections::HashMap, rc::Rc};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct RealtimeDisplayProps {
    pub game_data: Rc<Realtime>,
}

#[derive(PartialEq, Properties)]
pub struct MakeTableHeaderProps {
    pub map: HashMap<String, String>,
    pub champion_id: Option<String>,
    pub instance_name: String,
}

#[function_component(MakeTableHeader)]
fn make_table_header(props: &MakeTableHeaderProps) -> Html {
    html! {
        <>
            {
                props.map.iter().map(|(key, _)| {
                    let first_char = key.chars().next().unwrap();
                    let remaining = &key[first_char.len_utf8()..];
                    let img_path = if props.instance_name == "abilities" {
                        if first_char == 'C' || first_char == 'A' {
                            format!(
                                "img/{}/{}.png",
                                props.instance_name.clone(),
                                first_char
                            )
                        }
                        else {
                            format!(
                                "img/{}/{}{}.png",
                                props.instance_name.clone(),
                                props.champion_id.clone().unwrap(),
                                first_char
                            )
                        }
                    } else {
                        format!(
                            "img/{}/{}.png",
                            props.instance_name,
                            key.clone()
                        )
                    };
                    let text_content = if props.instance_name == "abilities" {
                        if !remaining.is_empty() {
                            html! {
                                <>
                                    {first_char}
                                    <sub>{remaining.replace("_", "")}</sub>
                                </>
                            }
                        } else {
                            html! {
                                {first_char}
                            }
                        }
                    } else {
                        html!{}
                    };
                    html! {
                        <th class="p-2">
                            <div class="flex items-center justify-center relative">
                                <img
                                    class="w-8 h-8"
                                    src={img_path}
                                />
                                <span class="text-shadow text-white">{ text_content }</span>
                            </div>
                        </th>
                    }
                })
                .collect::<Html>()
            }
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct MakeTableBodyProps {
    pub damages: HashMap<String, InstanceDamage>,
    pub ordered_instances: Vec<String>,
}

#[function_component(MakeTableBody)]
fn make_table_body(props: &MakeTableBodyProps) -> Html {
    html! {
        <>
            {
                props.ordered_instances.iter().map(|key| {
                    let value = props.damages.get(key).unwrap();
                    html! {
                        <td class="h-12 p-2 text-center">
                            <span class={value.damage_type.clone()}>
                                { format!("{:.0}",value.minimum_damage) }
                            </span>
                        </td>
                    }
                })
                .collect::<Html>()
            }
        </>
    }
}

#[function_component(RealtimeDisplay)]
pub fn realtime_display(props: &RealtimeDisplayProps) -> Html {
    let current_player = &props.game_data.current_player;
    let enemies = &props.game_data.enemies;

    html! {
        <div>
            <div>{"Loaded Realtime app"}</div>
            <table>
                <thead>
                    <tr class="bg-[#1e293b]">
                        <td class="h-12 p-2"></td>
                        <MakeTableHeader
                            champion_id={current_player.champion_id.clone()}
                            map={current_player.damaging_abilities.clone()}
                            instance_name={"abilities"}
                        />
                        <MakeTableHeader
                            champion_id={Option::<String>::None}
                            map={current_player.damaging_items.clone()}
                            instance_name={"items"}
                        />
                        <MakeTableHeader
                            champion_id={Option::<String>::None}
                            map={current_player.damaging_runes.clone()}
                            instance_name={"runes"}
                        />
                    </tr>
                </thead>
                <tbody>
                    {
                        enemies.iter().map(|enemy| {
                            html! {
                                <tr class="odd:bg-[#111827] even:bg-[#1a253b]">
                                    <td class="h-12 p-2">
                                        <img
                                            src={format!("img/champions/{}.png", enemy.champion_id)}
                                            class="w-8 h-8"
                                        />
                                    </td>
                                    <MakeTableBody
                                        damages={enemy.damages.abilities.clone()}
                                        ordered_instances={current_player.damaging_abilities.keys().cloned().collect::<Vec<String>>()}
                                    />
                                    <MakeTableBody
                                        damages={enemy.damages.items.clone()}
                                        ordered_instances={current_player.damaging_items.keys().cloned().collect::<Vec<String>>()}
                                    />
                                    <MakeTableBody
                                        damages={enemy.damages.runes.clone()}
                                        ordered_instances={current_player.damaging_runes.keys().cloned().collect::<Vec<String>>()}
                                    />
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}
