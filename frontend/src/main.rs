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

use std::rc::Rc;

use context::core::CoreProvider;
use pages::{
    about::About, calculator::CalculatorDisplay, formulas::Formulas, realtime::RealtimeDisplay,
};

use components::sidebar::Sidebar;
use model::{realtime::Realtime, realtime_example::makeup_example};
use yew::prelude::*;

pub const IMG_CDN: &str = "http://localhost:8082/cdn";

#[function_component(App)]
fn app() -> Html {
    let selected_page = use_state(|| 2usize);
    html! {
        <CoreProvider>
            <div class="grid grid-cols-[auto_1fr]">
                <Sidebar state_handler={selected_page.clone()} />
                {
                    match *selected_page {
                        // 1 => html! { <RealtimeDisplay /> },
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
