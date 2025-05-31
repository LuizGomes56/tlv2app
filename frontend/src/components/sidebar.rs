use yew::prelude::*;

use crate::{IMG_CDN, img::header::*};

fn make_header_li(img: Html, text: &str, selected: bool) -> Html {
    html! {
        <li class={
            format!(
                "relative flex items-center uppercase font-thin text-sm gap-2 h-8 font-semibold {}",
                if selected { "text-[#d9636c]" } else { "text-slate-400" }
            )}>
            {
                if selected {
                    html! {
                        <div class="absolute h-full w-1 rounded-r-xl bg-[#d9636c]" />
                    }
                }
                else {
                    html! {}
                }
            }
            <div class="px-4 flex items-center gap-2">
                <div class="text-shadow w-4 h-4 flex-shrink-0 text-inherit">
                    { img }
                </div>
                <span class="text-inherit text-shadow">{ text }</span>
            </div>
        </li>
    }
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <nav class="h-screen w-48 bg-slate-900">
            <div class="flex items-center gap-4 p-4 mb-6 mt-2">
                <img
                    class="w-6 h-6 flex-shrink-0"
                    src={format!("{}/other/league_logo.svg", IMG_CDN)}
                    alt=""
                />
                <img
                    class="h-8"
                    src={format!("{}/other/league.svg", IMG_CDN)}
                    alt=""
                />
            </div>
            <ul class="flex flex-col gap-2">
                { make_header_li(dashboard_svg(), "Dashboard", false) }
                { make_header_li(play_svg(), "Realtime", false) }
                { make_header_li(calculator_svg(), "Calculator", false) }
            </ul>
            <hline class="flex m-4 bg-slate-600 h-px rounded-full" />
            <ul class="flex flex-col gap-2">
                { make_header_li(about_svg(), "About", false) }
                { make_header_li(formulas_svg(), "Formulas", true) }
                { make_header_li(github_svg(), "Github", false) }
            </ul>
        </nav>
    }
}
