use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="flex flex-col gap-8 p-6 text-white max-h-screen overflow-y-scroll w-full">
            <div class="flex flex-col gap-2">
                <h1 class="font-bold text-2xl">{ "About TutorLoLv2" }</h1>
                <h3 class="text-sm text-slate-200">{ "" }</h3>
            </div>
            <div class="flex flex-col gap-4">
            </div>
        </div>
    }
}
