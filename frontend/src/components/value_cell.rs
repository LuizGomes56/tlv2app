use yew::prelude::*;

use crate::IMG_CDN;

pub fn value_cell(image_source: &str, value: String, oninput: Callback<InputEvent>) -> Html {
    html! {
            <div class="grid grid-cols-[auto_1fr] gap-2">
                <img
                    class="h-8 w-8 aspect-square"
                    src={format!("{}/other/{}", IMG_CDN, image_source)}
                    alt="Cell"
                />
                <input
                    oninput={oninput}
                    class="w-full bg-custom-800 h-8 text-center"
                    type="number"
                    value={value}
                    min="0"
                    max="1"
                    aria-label="Ability"
                />
            </div>
    }
}
