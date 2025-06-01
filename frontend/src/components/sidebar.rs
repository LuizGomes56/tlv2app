use yew::prelude::*;

use crate::{IMG_CDN, img::header::*};

fn make_header_li(
    index: usize,
    state_handler: &UseStateHandle<usize>,
    img: Html,
    text: &str,
) -> Html {
    let selected = **state_handler == index;

    html! {
        <li
            class={
                format!(
                    "cursor-pointer relative rounded-md w-full flex items-center h-10 font-semibold px-4 gap-2 {}",
                    if selected { "bg-zinc-900 text-white" } else { "text-[#8E8F93]" }
                )
            }
            onclick={{
                let state_handler = state_handler.clone();
                Callback::from(move |_| state_handler.set(index))
            }}
        >
            <div class="flex items-center gap-2">
                <div class="text-shadow w-5 h-5 flex-shrink-0 text-inherit">
                    { img }
                </div>
                <span class="text-inherit">{ text }</span>
            </div>
        </li>
    }
}

#[derive(PartialEq, Properties)]
pub struct SidebarProps {
    pub state_handler: UseStateHandle<usize>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    html! {
        <nav style="box-shadow: 5px 0px 10px black;" class="h-screen bg-zinc-950 w-48">
            <div class="flex items-center gap-3 p-4 mb-6 mt-3 justify-center">
                <img
                    class="w-8 h-8 flex-shrink-0"
                    src={format!("{}/other/league_logo.svg", IMG_CDN)}
                    alt=""
                />
                <img
                    class="h-10"
                    src={format!("{}/other/league.svg", IMG_CDN)}
                    alt=""
                />
            </div>
            <ol class="flex flex-col gap-12">
                <ul class="flex flex-col gap-2 px-4">
                    <li class="text-zinc-200 font-semibold px-4 mb-2 w-full">
                        { "GAMEPLAY" }
                    </li>
                    { make_header_li(0, &props.state_handler, dashboard_svg(), "Dashboard") }
                    { make_header_li(1, &props.state_handler, play_svg(), "Realtime") }
                    { make_header_li(2, &props.state_handler, calculator_svg(), "Calculator") }
                </ul>
                <ul class="flex flex-col gap-2 px-4">
                    <li class="text-zinc-200 font-semibold px-4 mb-2 w-full">
                        { "APPLICATION" }
                    </li>
                    { make_header_li(3, &props.state_handler, about_svg(), "About") }
                    { make_header_li(4, &props.state_handler, formulas_svg(), "Formulas") }
                    { make_header_li(5, &props.state_handler, github_svg(), "Github") }
                </ul>
            </ol>
        </nav>
    }
}
