use crate::{
    components::base_table::{MakeTableBody, MakeTableHeader, champion_td},
    model::traits::{CurrentPlayerLike, EnemyLike},
};

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BaseTableProps<T, U>
where
    T: CurrentPlayerLike + PartialEq,
    U: EnemyLike + PartialEq,
{
    pub current_player: T,
    pub enemies: Vec<U>,
    pub item_id: String,
}

#[function_component(ComparisonTable)]
pub fn comparison_table<T: CurrentPlayerLike + PartialEq, U: EnemyLike + PartialEq>(
    props: &BaseTableProps<T, U>,
) -> Html {
    let champion_id = props.current_player.get_champion_id();
    let (damaging_abilities, damaging_items, damaging_runes) =
        props.current_player.get_damaging_instances();

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
                        let enemy_champion_id=  enemy.get_champion_id();
                        let damages = enemy.get_damages();

                        if let Some(final_damage) = damages.compared_items.get(&props.item_id) {
                            html! {
                                <tr>
                                    {champion_td(&enemy_champion_id)}
                                    <MakeTableBody
                                        damages={final_damage.abilities.damages.clone()}
                                        ordered_instances={damaging_abilities.keys().cloned().collect::<Vec<String>>()}
                                    />
                                    <MakeTableBody
                                        damages={final_damage.items.damages.clone()}
                                        ordered_instances={damaging_items.keys().cloned().collect::<Vec<String>>()}
                                    />
                                    <MakeTableBody
                                        damages={final_damage.runes.damages.clone()}
                                        ordered_instances={damaging_runes.keys().cloned().collect::<Vec<String>>()}
                                    />
                                </tr>
                            }
                        } else {
                            html! {}
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
