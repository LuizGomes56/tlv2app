use yew::prelude::*;

use crate::BACKEND_URL;

pub fn value_cell(image_source: &str, value: String, oninput: Callback<InputEvent>) -> Html {
    html! {
        <div class="grid grid-cols-[auto_1fr] gap-2">
            <img
                class="h-8 min-w-8 aspect-square"
                src={format!("{}/cdn/other/{}", BACKEND_URL, image_source)}
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
