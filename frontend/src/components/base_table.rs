use crate::model::realtime::{CurrentPlayer, Enemy, InstanceDamage};
use std::collections::HashMap;
use yew::prelude::*;

pub fn champion_td(champion_id: &str) -> Html {
    html! {
        <td>
            <img
                src={format!("img/champions/{}.png", champion_id)}
                alt="Champion"
            />
        </td>
    }
}

#[derive(PartialEq, Properties)]
pub struct MakeTableHeaderProps {
    pub map: HashMap<String, String>,
    pub champion_id: Option<String>,
    pub instance_name: String,
}

pub fn create_image(keyname: &str, champion_id: Option<String>, instance_name: &str) -> Html {
    let first_char = keyname.chars().next().unwrap();
    let remaining = &keyname[first_char.len_utf8()..];
    let is_attack_related = first_char == 'C' || first_char == 'A';
    let img_path = if instance_name == "abilities" {
        if is_attack_related {
            format!("img/{}/{}.png", instance_name, first_char)
        } else {
            format!(
                "img/{}/{}{}.png",
                instance_name,
                champion_id.unwrap_or_default(),
                first_char
            )
        }
    } else {
        format!("img/{}/{}.png", instance_name, keyname)
    };
    let text_content = if instance_name == "abilities" && !is_attack_related {
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
        html! {}
    };
    html! {
        <>
            <img
                src={img_path}
                alt="Instance Icon"
            />
            <span class="text-shadow">{ text_content }</span>
        </>
    }
}

#[function_component(MakeTableHeader)]
fn make_table_header(props: &MakeTableHeaderProps) -> Html {
    html! {
        <>
            {
                props.map.iter().map(|(key, _)| {
                    html! {
                        <th>
                            <div>
                                {create_image(
                                    key,
                                    props.champion_id.clone(),
                                    &props.instance_name
                                )}
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
                        <td>
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

#[derive(PartialEq, Properties)]
pub struct BaseTableProps {
    pub current_player: CurrentPlayer,
    pub enemies: Vec<Enemy>,
}

#[function_component(BaseTable)]
pub fn base_table(props: &BaseTableProps) -> Html {
    html! {
        <table class="w-full">
            <thead>
                <tr>
                    <th></th>
                    <MakeTableHeader
                        champion_id={props.current_player.champion_id.clone()}
                        map={props.current_player.damaging_abilities.clone()}
                        instance_name={"abilities"}
                    />
                    <MakeTableHeader
                        champion_id={Option::<String>::None}
                        map={props.current_player.damaging_items.clone()}
                        instance_name={"items"}
                    />
                    <MakeTableHeader
                        champion_id={Option::<String>::None}
                        map={props.current_player.damaging_runes.clone()}
                        instance_name={"runes"}
                    />
                </tr>
            </thead>
            <tbody>
                {
                    props.enemies.iter().map(|enemy| {
                        html! {
                            <tr>
                                {champion_td(&enemy.champion_id)}
                                <MakeTableBody
                                    damages={enemy.damages.abilities.clone()}
                                    ordered_instances={props.current_player.damaging_abilities.keys().cloned().collect::<Vec<String>>()}
                                />
                                <MakeTableBody
                                    damages={enemy.damages.items.clone()}
                                    ordered_instances={props.current_player.damaging_items.keys().cloned().collect::<Vec<String>>()}
                                />
                                <MakeTableBody
                                    damages={enemy.damages.runes.clone()}
                                    ordered_instances={props.current_player.damaging_runes.keys().cloned().collect::<Vec<String>>()}
                                />
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
