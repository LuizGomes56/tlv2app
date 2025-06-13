use crate::{
    BACKEND_URL,
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
                src={format!("{}/cdn/champions/{}.png", BACKEND_URL, champion_id)}
                alt="Champion"
            />
        </td>
    }
}

pub fn create_image(keyname: &str, champion_id: Option<String>, instance_name: &str) -> Html {
    let first_char = keyname.chars().next().unwrap_or_default();
    let remaining = &keyname[first_char.len_utf8()..];
    let is_attack_related = first_char == 'C' || first_char == 'A';
    let img_path = if instance_name == "abilities" {
        if is_attack_related {
            format!("{}/cdn/{}/{}.png", BACKEND_URL, instance_name, first_char)
        } else {
            format!(
                "{}/cdn/{}/{}{}.png",
                BACKEND_URL,
                instance_name,
                champion_id.unwrap_or_default(),
                first_char
            )
        }
    } else {
        format!("{}/cdn/{}/{}.png", BACKEND_URL, instance_name, keyname)
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
            <span class={"img-letter"}>{ text_content }</span>
        </>
    }
}

pub fn make_table_header(
    champion_id: Option<String>,
    map: HashMap<String, String>,
    instance_name: &str,
) -> Html {
    let mut sorted_map = map.iter().collect::<Vec<_>>();
    sorted_map.sort_by(|a, b| a.0.cmp(&b.0));
    html! {
        <>
            {
                sorted_map.into_iter().map(|(key, _)| {
                    html! {
                        <th>
                            <div>
                                {
                                    create_image(
                                        key,
                                        champion_id.clone(),
                                        &instance_name
                                    )
                                }
                            </div>
                        </th>
                    }
                })
                .collect::<Html>()
            }
        </>
    }
}

pub fn make_table_body(damages: &DamageLike) -> Html {
    let mut sorted_damages = damages.iter().collect::<Vec<_>>();
    sorted_damages.sort_by(|a, b| a.0.cmp(&b.0));
    html! {
        sorted_damages.into_iter().map(|(_, value)| {
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
                            <div class={"flex flex-col"}>
                                <p>{ text }</p>
                                <p class={"text-zinc-400 text-[11px]"}>{ subtext }</p>
                            </div>
                        }
                    </span>
                </td>
            }
        })
        .collect::<Html>()
    }
}

pub fn base_table<T, U>(current_player: &T, enemies: &Vec<U>) -> Html
where
    T: CurrentPlayerLike,
    U: EnemyLike,
{
    let (damaging_abilities, damaging_items, damaging_runes) =
        current_player.get_damaging_instances();
    let champion_id = current_player.get_champion_id();
    html! {
        <table class={"w-full"}>
            <thead>
                <tr>
                    <th></th>
                    {
                        make_table_header(
                            Some(champion_id),
                            damaging_abilities,
                            "abilities"
                        )
                    }
                    {
                        make_table_header(
                            Option::<String>::None,
                            damaging_items,
                            "items"
                        )
                    }
                    {
                        make_table_header(
                            Option::<String>::None,
                            damaging_runes,
                            "runes"
                        )
                    }
                </tr>
            </thead>
            <tbody>
                {
                    enemies.iter().map(|enemy| {
                        let damages = enemy.get_damages();
                        let enemy_champion_id = enemy.get_champion_id();
                        html! {
                            <tr>
                                { champion_td(&enemy_champion_id) }
                                { make_table_body(&damages.abilities) }
                                { make_table_body(&damages.items) }
                                { make_table_body(&damages.runes) }
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
