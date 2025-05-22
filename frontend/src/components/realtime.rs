use crate::{
    components::{
        base_table::BaseTable,
        stacker::{StackDropper, StackSelector, Stacker},
    },
    model::realtime::Realtime,
};
use std::rc::Rc;
use yew::prelude::*;

use super::stacker::StackInstance;

#[derive(PartialEq, Properties)]
pub struct RealtimeDisplayProps {
    pub game_data: Rc<Realtime>,
}

#[function_component(RealtimeDisplay)]
pub fn realtime_display(props: &RealtimeDisplayProps) -> Html {
    let current_player = &props.game_data.current_player;
    let enemies = &props.game_data.enemies;

    let stack: UseStateHandle<Vec<StackInstance>> = use_state(|| Vec::<StackInstance>::new());

    html! {
        <div class="flex flex-col gap-4">
            <div class="overflow-auto">
                <BaseTable
                    current_player={current_player.clone()}
                    enemies={enemies.clone()}
                />
            </div>
            <div class="p-4 shadow-container">
                <div class="max-w-min flex flex-col gap-4">
                    <StackSelector
                        stack={stack.clone()}
                        champion_id={current_player.champion_id.clone()}
                        abilities={current_player.damaging_abilities.clone()}
                        items={current_player.damaging_items.clone()}
                        runes={current_player.damaging_runes.clone()}
                    />
                    <div class="overflow-auto">
                        <Stacker
                            stack={(*stack).clone()}
                            enemies={enemies.clone()}
                        />
                    </div>
                    <StackDropper
                        champion_id={current_player.champion_id.clone()}
                        stack={stack.clone()}
                    />
                </div>
            </div>
        </div>
    }
}
