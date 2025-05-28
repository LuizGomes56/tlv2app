use yew::prelude::*;

use crate::IMG_CDN;

#[derive(PartialEq, Properties)]
pub struct ValueCellProps {
    pub image_source: String,
    pub value: String,
    pub oninput: Callback<InputEvent>,
}

#[function_component(ValueCell)]
pub fn value_cell(props: &ValueCellProps) -> Html {
    html! {
            <div class="grid grid-cols-[auto_1fr] gap-2">
                <img
                    class="h-8 w-8 aspect-square"
                    src={format!("{}/other/{}", IMG_CDN, props.image_source)}
                    alt="Cell"
                />
                <input
                    oninput={props.oninput.clone()}
                    class="w-full bg-slate-800 h-8 text-center"
                    type="number"
                    value={props.value.clone()}
                    min="0"
                    max="1"
                    aria-label="Ability"
                />
            </div>
    }
}
