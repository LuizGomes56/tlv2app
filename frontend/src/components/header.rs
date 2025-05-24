use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="sticky w-full flex items-center p-4 bg-slate-900 shadow-container">
            {
                [
                    ("Repository", "img/other/github.svg"),
                    ("Realtime", "img/other/controller.svg"),
                    ("Calculator", "img/other/calculator.svg"),
                    ("Announcements", "img/other/league_logo.svg"),
                ].into_iter().map(|(text, icon)| {
                    html! {
                        <div class="flex flex-1 items-center gap-4">
                            <img
                                src={icon}
                                alt="Header Icon"
                                class="w-5 h-5"
                            />
                            <span class="text-shadow">{text}</span>
                        </div>
                    }
                }).collect::<Html>()
            }
        </header>
    }
}
