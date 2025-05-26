use yew::prelude::*;

use crate::IMG_CDN;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="sticky w-full flex items-center p-4 bg-slate-900 shadow-container">
            {
                [
                    ("Repository", format!("{}/other/github.svg", IMG_CDN)),
                    ("Realtime", format!("{}/other/controller.svg", IMG_CDN)),
                    ("Calculator", format!("{}/other/calculator.svg", IMG_CDN)),
                    ("Announcements", format!("{}/other/league_logo.svg", IMG_CDN)),
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
