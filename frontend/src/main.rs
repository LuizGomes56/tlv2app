#![allow(dead_code)]
#![allow(unused_imports)]

mod components;
mod consts;
mod hooks;
mod img;
mod macros;
mod model;
mod pages;
mod tauriapp;

use std::rc::Rc;

use pages::{calculator::CalculatorDisplay, formulas::Formulas, realtime::RealtimeDisplay};

use components::sidebar::Sidebar;
use model::{realtime::Realtime, realtime_example::makeup_example};
use yew::prelude::*;

pub const IMG_CDN: &str = "http://localhost:8082/cdn";

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="flex">
            <Sidebar />
            <Formulas />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
