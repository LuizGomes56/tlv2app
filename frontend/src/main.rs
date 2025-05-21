use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, console};
use yew::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
unsafe extern "C" {
    #[wasm_bindgen(js_name = invokeSendCode, catch)]
    pub async fn invoke_send_code() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invokeStartGame, catch)]
    pub async fn invoke_start_game() -> Result<JsValue, JsValue>;
}

fn start_game() {
    spawn_local(async move {
        match invoke_start_game().await {
            Ok(_) => {
                console::log_1(&JsValue::from("Game started"));
            }
            Err(e) => {
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
                console::log_1(&code_value);
                game_code.set(code_value.as_string().unwrap());
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

    let onclick = Callback::from(move |_| {
        start_game();
    });

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
