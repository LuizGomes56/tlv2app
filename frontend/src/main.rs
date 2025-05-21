mod model;

use model::realtime::Realtime;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, console};
use yew::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
unsafe extern "C" {
    #[wasm_bindgen(js_name = invoke_send_code, catch)]
    pub async fn invoke_send_code() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invoke_start_game, catch)]
    pub async fn invoke_start_game() -> Result<JsValue, JsValue>;
}

fn start_game(game_data: UseStateHandle<Option<Realtime>>) {
    spawn_local(async move {
        match invoke_start_game().await {
            Ok(value) => {
                console::log_1(&value);
                let json_string = value.as_string().unwrap_or_default();
                match serde_json::from_str(&json_string) {
                    Ok(realtime_data) => {
                        game_data.set(realtime_data);
                    }
                    Err(e) => {
                        console::log_1(&JsValue::from_str(e.to_string().as_str()));
                    }
                }
            }
            Err(e) => {
                let error_msg = JsValue::from_str(
                    "É necessário estar instalar o aplicativo nativo para usar o Realtime.",
                );
                console::log_1(&error_msg);
                console::log_1(&e);
            }
        }
    });
}

fn get_code(game_code: UseStateHandle<String>) {
    spawn_local(async move {
        let code = invoke_send_code().await;
        match code {
            Ok(code_value) => {
                if code_value.is_undefined() {
                    console::log_1(&JsValue::from_str("O aplicativo nativo não está em uso"));
                    game_code.set("??????".to_string());
                } else {
                    game_code.set(code_value.as_string().unwrap());
                }
            }
            Err(e) => {
                console::log_1(&e);
            }
        }
    });
}

#[function_component(App)]
fn app() -> Html {
    let game_code = use_state(|| String::new());
    let game_data = use_state(|| Option::<Realtime>::None);

    let oninput = {
        let game_code = game_code.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            game_code.set(input.value());
        })
    };

    {
        let game_code = game_code.clone();
        use_effect_with((), move |_| {
            get_code(game_code);
            || ()
        });
    }

    let onclick = {
        let game_data = game_data.clone();
        Callback::from(move |_: MouseEvent| {
            start_game(game_data.clone());
        })
    };

    html! {
        <div>
            <h1>{ "Hello Worlds of yew" }</h1>
            <input type="text" {oninput} />
            <h2>{ format!("Code: {}", *game_code) }</h2>
            <button {onclick}>{ "Start Game" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
