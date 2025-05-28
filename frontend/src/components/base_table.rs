use crate::{
    IMG_CDN,
    model::{
        realtime::DamageLike,
        traits::{CurrentPlayerLike, EnemyLike},
    },
};
use std::collections::HashMap;
use yew::prelude::*;

pub fn champion_td(champion_id: &str) -> Html {
    html! {
        <td>
            <img
                src={format!("{}/champions/{}.png", IMG_CDN, champion_id)}
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
            format!("{}/{}/{}.png", IMG_CDN, instance_name, first_char)
        } else {
            format!(
                "{}/{}/{}{}.png",
                IMG_CDN,
                instance_name,
                champion_id.unwrap_or_default(),
                first_char
            )
        }
    } else {
        format!("{}/{}/{}.png", IMG_CDN, instance_name, keyname)
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
            <span class="img-letter">{ text_content }</span>
        </>
    }
}

#[function_component(MakeTableHeader)]
pub fn make_table_header(props: &MakeTableHeaderProps) -> Html {
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
    pub damages: DamageLike,
    pub ordered_instances: Vec<String>,
}

#[function_component(MakeTableBody)]
pub fn make_table_body(props: &MakeTableBodyProps) -> Html {
    html! {
        props.ordered_instances.iter().map(|key| {
            let value = props.damages.get(key).unwrap();
            let text = if value.maximum_damage > 0.0 {
                format!("{:.0} - {:.0}", value.minimum_damage, value.maximum_damage)
            } else {
                format!("{:.0}", value.minimum_damage)
            };
            let mut subtext = String::new();
            if let Some(min_dmg_change) = value.min_dmg_change {
                subtext.push_str(format!("{:.0}", min_dmg_change).as_str());
            }
            if let Some(max_dmg_change) = value.max_dmg_change {
                if max_dmg_change > 0.0 {
                    subtext.push_str(format!(" - {:.0}", max_dmg_change).as_str());
                }
            };
            html! {
                <td>
                    <span class={value.damage_type.clone()}>
                        if subtext.is_empty() {
                            { text }
                        } else {
                            <div class="flex flex-col">
                                <p>{ text }</p>
                                <p class="text-zinc-400 text-[11px]">{ subtext }</p>
                            </div>
                        }
                    </span>
                </td>
            }
        })
        .collect::<Html>()
    }
}

#[derive(PartialEq, Properties)]
pub struct BaseTableProps<T, U>
where
    T: CurrentPlayerLike + PartialEq,
    U: EnemyLike + PartialEq,
{
    pub current_player: T,
    pub enemies: Vec<U>,
}

#[function_component(BaseTable)]
pub fn base_table<T, U>(props: &BaseTableProps<T, U>) -> Html
where
    T: CurrentPlayerLike + PartialEq,
    U: EnemyLike + PartialEq,
{
    let (damaging_abilities, damaging_items, damaging_runes) =
        props.current_player.get_damaging_instances();
    let champion_id = props.current_player.get_champion_id();
    html! {
        <table class="w-full">
            <thead>
                <tr>
                    <th></th>
                    <MakeTableHeader
                        champion_id={champion_id.clone()}
                        map={damaging_abilities.clone()}
                        instance_name={"abilities"}
                    />
                    <MakeTableHeader
                        champion_id={Option::<String>::None}
                        map={damaging_items.clone()}
                        instance_name={"items"}
                    />
                    <MakeTableHeader
                        champion_id={Option::<String>::None}
                        map={damaging_runes.clone()}
                        instance_name={"runes"}
                    />
                </tr>
            </thead>
            <tbody>
                {
                    props.enemies.iter().map(|enemy| {
                        let damages = enemy.get_damages();
                        let enemy_champion_id = enemy.get_champion_id();
                        html! {
                            <tr>
                                {champion_td(&enemy_champion_id)}
                                <MakeTableBody
                                    damages={damages.abilities.clone()}
                                    ordered_instances={damaging_abilities.keys().cloned().collect::<Vec<String>>()}
                                />
                                <MakeTableBody
                                    damages={damages.items.clone()}
                                    ordered_instances={damaging_items.keys().cloned().collect::<Vec<String>>()}
                                />
                                <MakeTableBody
                                    damages={damages.runes.clone()}
                                    ordered_instances={damaging_runes.keys().cloned().collect::<Vec<String>>()}
                                />
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
