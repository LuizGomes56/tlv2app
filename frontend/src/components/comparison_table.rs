use crate::{
    components::base_table::{champion_td, make_table_body, make_table_header},
    model::traits::{CurrentPlayerLike, EnemyLike},
};

use yew::prelude::*;

pub fn comparison_table<T, U>(current_player: &T, enemies: &Vec<U>, item_id: String) -> Html
where
    T: CurrentPlayerLike,
    U: EnemyLike,
{
    let champion_id = current_player.get_champion_id();
    let (damaging_abilities, damaging_items, damaging_runes) =
        current_player.get_damaging_instances();

    html! {
        <table class="w-full">
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
                        let enemy_champion_id = enemy.get_champion_id();
                        let damages = enemy.get_damages();

                        if let Some(final_damage) = damages.compared_items.get(&item_id) {
                            html! {
                                <tr>
                                    {champion_td(&enemy_champion_id)}
                                    { make_table_body(&final_damage.abilities.damages) }
                                    { make_table_body(&final_damage.items.damages) }
                                    { make_table_body(&final_damage.runes.damages) }
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
