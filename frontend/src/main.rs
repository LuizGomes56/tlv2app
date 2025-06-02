#![allow(dead_code)]
#![allow(unused_imports)]

mod components;
mod consts;
mod context;
mod hooks;
mod img;
mod macros;
mod model;
mod pages;
mod tauriapp;

use std::{ops::Deref, rc::Rc};

use context::core::CoreProvider;
use pages::{
    about::About, calculator::CalculatorDisplay, formulas::Formulas, realtime::RealtimeDisplay,
};

use components::sidebar::Sidebar;
use model::{realtime::Realtime, realtime_example::makeup_example};
use tauriapp::invokers::get_code;
use yew::prelude::*;

pub const IMG_CDN: &str = "http://localhost:8082/cdn";
pub const MAX_FAILURES: usize = 10;

#[function_component(App)]
fn app() -> Html {
    let game_code = use_state(|| String::from("000000"));
    let selected_page = use_state(|| 1usize);

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
                        1 => html! { <RealtimeDisplay game_code={game_code.deref().clone()} /> },
                        2 => html! { <CalculatorDisplay /> },
                        3 => html! { <About /> },
                        4 => html! { <Formulas /> },
                        // 5 => html! { <Github /> },
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
