#![allow(dead_code)]
#![allow(unused_imports)]

mod components;
mod context;
mod externcalls;
mod hooks;
mod img;
mod macros;
mod model;
mod pages;

use std::{ops::Deref, rc::Rc};

use context::core::CoreProvider;
use pages::{
    about::about, calculator::CalculatorDisplay, formulas::Formulas, github::github,
    realtime::RealtimeDisplay,
};

use components::sidebar::Sidebar;
use externcalls::invokers::get_code;
use model::realtime::Realtime;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::pages::dashboard::dashboard;

// pub const BACKEND_URL: &str = "http://localhost:8082";
pub const BACKEND_URL: &str = "https://v2.tutorlol.com";
pub const MAX_FAILURES: usize = 10;

#[function_component(App)]
fn app() -> Html {
    let game_code = use_state(|| 0usize);
    let selected_page = use_state(|| 1usize);

    {
        let game_code = game_code.clone();
        use_effect_with((), move |_| {
            get_code(game_code);
        })
    };

    html! {
        <CoreProvider>
            <div class={"grid grid-cols-[auto_1fr]"}>
                <Sidebar state_handler={selected_page.clone()} />
                {
                    match *selected_page {
                        0 => html! { dashboard() },
                        1 => html! { <RealtimeDisplay game_code_state={game_code} /> },
                        2 => html! { <CalculatorDisplay /> },
                        3 => html! { about() },
                        4 => html! { <Formulas /> },
                        5 => html! { github() },
                        _ => html! {
                            <div class={"flex flex-col gap-12 max-h-screen overflow-y-auto p-12"}>
                                <h1 class={"font-bold text-4xl text-white"}>{ "Invalid page [404]" }</h1>
                            </div>
                        },
                    }
                }
            </div>
        </CoreProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
