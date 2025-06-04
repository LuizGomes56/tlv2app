use std::{cell::RefCell, ops::Deref, rc::Rc};

use gloo::timers::callback::Interval;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::{MAX_FAILURES, model::realtime::Realtime};

#[wasm_bindgen(module = "/public/glue.js")]
unsafe extern "C" {
    #[wasm_bindgen(js_name = invokeGetGameCode)]
    pub async fn invoke_get_game_code() -> JsValue;

    #[wasm_bindgen(js_name = invokeGetRealtimeGame, catch)]
    pub async fn invoke_get_realtime_game(game_code: usize) -> Result<JsValue, JsValue>;
}

pub fn get_realtime_game(
    game_code: usize,
    game_data: UseStateHandle<Option<Rc<Realtime>>>,
    counter: Rc<RefCell<usize>>,
) {
    let counter_clone = Rc::clone(&counter);

    spawn_local(async move {
        match invoke_get_realtime_game(game_code).await {
            Ok(value) => {
                if let Some(json_string) = value.as_string() {
                    if !json_string.is_empty() {
                        match serde_json::from_str(&json_string) {
                            Ok(realtime_data) => {
                                game_data.set(Some(Rc::new(realtime_data)));
                                *counter_clone.borrow_mut() = 0;
                            }
                            Err(e) => {
                                console::log_1(&format!("Erro de parsing: {}", e).into());
                                *counter_clone.borrow_mut() += 1;
                            }
                        }
                    } else {
                        console::log_1(&"Resposta vazia".into());
                        *counter_clone.borrow_mut() += 1;
                    }
                } else {
                    console::log_1(&"Nenhuma string na resposta".into());
                    *counter_clone.borrow_mut() += 1;
                }
            }
            Err(e) => {
                console::log_1(&"Erro na requisição".into());
                console::log_1(&e);
                *counter_clone.borrow_mut() += 1;
            }
        }
    });
}

pub fn get_code(game_code: UseStateHandle<usize>) {
    spawn_local(async move {
        let code = invoke_get_game_code().await;
        if code.is_undefined() {
            console::log_1(&"O aplicativo nativo não está em uso".into());
        } else {
            game_code.set(code.as_f64().unwrap_or_default() as usize);
        }
    });
}
