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
use model::{realtime::Realtime, realtime_example::makeup_example};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub const BACKEND_URL: &str = "http://localhost:8082";
pub const MAX_FAILURES: usize = 10;

#[function_component(App)]
fn app() -> Html {
    let game_code = use_state(|| String::from("000000"));
    let selected_page = use_state(|| 4usize);

    {
        let game_code = game_code.clone();
        use_effect_with((), move |_| {
            get_code(game_code);
        })
    };

    html! {
        <CoreProvider>
            <div class="grid grid-cols-[auto_1fr]">
                <Sidebar state_handler={selected_page.clone()} />
                {
                    match *selected_page {
                        1 => html! { <RealtimeDisplay game_code_state={game_code} /> },
                        2 => html! { <CalculatorDisplay /> },
                        3 => html! { about() },
                        4 => html! { <Formulas /> },
                        5 => html! { github() },
                        _ => html! {},
                    }
                }
            </div>
        </CoreProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
