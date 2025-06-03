use yew::prelude::*;

use crate::{BACKEND_URL, model::realtime::ComparedItem};

pub fn comparison_header(value: &ComparedItem, item_id: &str) -> Html {
    let mut sorted_stats: Vec<_> = value.prettified_stats.iter().collect();
    sorted_stats.sort_by(|a, b| a.0.cmp(b.0));

    html! {
        <div class="flex flex-col p-3">
            <div class="flex justify-between items-center gap-4 pb-3 mb-2.5 border-b border-b border-b-zinc-600 w-full">
                <div class="flex items-center gap-4">
                    <img
                        class="w-8 h-8 aspect-square flex-shrink-0"
                        src={format!("{}/cdn/items/{}.png", BACKEND_URL, item_id)}
                        alt="Compared Item"
                    />
                    <span class="text-shadow font-bold">
                        {value.name.clone()}
                    </span>
                </div>
                <div class="flex items-center gap-1">
                    <img
                        class="w-4 h-4 aspect-square flex-shrink-0"
                        src={format!("{}/cdn/stats/GoldPer10Seconds.png", BACKEND_URL)}
                        alt="Gold Cost"
                    />
                    <span class="text-yellow-300 text-shadow">{value.gold_cost}</span>
                </div>
            </div>
            <div class="grid grid-cols-3">
                {sorted_stats.iter().map(|(stat_name, stat_value)| {
                    let stat_img_path = stat_name
                        .to_string()
                        .split(' ')
                        .collect::<Vec<&str>>()
                        .join("");
                    html! {
                        <div class="flex items-center gap-2">
                            <img
                                class="w-4 h-4 aspect-square flex-shrink-0"
                                src={format!("{}/cdn/stats/{}.png", BACKEND_URL, stat_img_path)}
                                alt="Stat"
                            />
                            <span class="text-sm text-zinc-300 text-shadow">
                                {format!("{} {}", stat_value.to_string().trim_end_matches(".0"), stat_name)}
                            </span>
                        </div>
                    }
                    })
                    .collect::<Html>()
                }
            </div>
        </div>
    }
}
