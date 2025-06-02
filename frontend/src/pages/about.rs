use yew::prelude::*;

pub fn about() -> Html {
    html! {
        <div class="flex flex-col gap-12 max-h-screen overflow-y-auto p-12">
            <h1 class="font-bold text-4xl text-white">{ "About this project" }</h1>
        </div>
    }
}
