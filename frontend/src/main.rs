mod components;
mod hooks;
mod macros;
mod model;
mod tauriapp;

use std::rc::Rc;

use components::{calculator::CalculatorDisplay, header::Header, realtime::RealtimeDisplay};
// use gloo::timers::callback::Interval;
use model::{realtime::Realtime, realtime_example::makeup_example};
// use tauriapp::invokers::get_realtime_game;
// use web_sys::HtmlInputElement;
use yew::prelude::*;

pub const IMG_CDN: &str = "http://localhost:8082/cdn";

#[function_component(App)]
fn app() -> Html {
    // let game_code = use_state(|| String::from("000000"));
    let game_data = use_state(|| Option::<Rc<Realtime>>::None);
    // let interval_handle = use_state(|| None);

    // let oninput = {
    //     let game_code = game_code.clone();

    //     Callback::from(move |e: InputEvent| {
    //         let input: HtmlInputElement = e.target_unchecked_into();
    //         game_code.set(input.value());
    //     })
    // };

    {
        // let game_code = game_code.clone();
        let game_data = game_data.clone();
        use_effect_with((), move |_| {
            makeup_example(game_data);
            // get_code(game_code);
            || ()
        });
    }

    // let start_game = {
    //     let game_data = game_data.clone();
    //     let interval_handle = interval_handle.clone();

    //     Callback::from(move |_| {
    //         if interval_handle.is_some() {
    //             return;
    //         }
    //         let game_data = game_data.clone();
    //         let handle = Interval::new(1000, move || {
    //             get_realtime_game(game_data.clone());
    //         });

    //         interval_handle.set(Some(handle));
    //     })
    // };

    // let stop_game = {
    //     let interval_handle = interval_handle.clone();

    //     Callback::from(move |_| {
    //         interval_handle.set(None);
    //     })
    // };

    html! {
        <div class="flex flex-col">
            <Header />
            // <input type="text" {oninput} />
            // <h2>{ format!("Code: {}", *game_code) }</h2>
            // <button onclick={start_game}>{ "Start Game" }</button>
            // <button onclick={stop_game}>{ "Stop Game" }</button>
            {
                if game_data.is_some() {
                    if let Some(data) = game_data.as_ref() {
                        html! {
                            <CalculatorDisplay />
                            // <RealtimeDisplay game_data={data} game_code={(*game_code).clone()} />
                        }
                    } else {
                        html! {}
                    }
                }
                else {
                    html! {}
                }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
