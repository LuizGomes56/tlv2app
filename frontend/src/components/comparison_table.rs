use crate::{
    components::base_table::{MakeTableBody, MakeTableHeader, champion_td},
    model::realtime::{CurrentPlayer, Enemy},
};

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BaseTableProps {
    pub current_player: CurrentPlayer,
    pub enemies: Vec<Enemy>,
    pub item_id: String,
}

#[function_component(ComparisonTable)]
pub fn comparison_table(props: &BaseTableProps) -> Html {
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
                                    damages={enemy.damages.compared_items.get(&props.item_id).unwrap().abilities.damages.clone()}
                                    ordered_instances={props.current_player.damaging_abilities.keys().cloned().collect::<Vec<String>>()}
                                />
                                <MakeTableBody
                                    damages={enemy.damages.compared_items.get(&props.item_id).unwrap().items.damages.clone()}
                                    ordered_instances={props.current_player.damaging_items.keys().cloned().collect::<Vec<String>>()}
                                />
                                <MakeTableBody
                                    damages={enemy.damages.compared_items.get(&props.item_id).unwrap().runes.damages.clone()}
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
