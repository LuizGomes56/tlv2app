use std::{cell::RefCell, ops::Deref, rc::Rc};

use gloo::timers::callback::Interval;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::{MAX_FAILURES, model::realtime::Realtime};

#[wasm_bindgen(module = "/public/glue.js")]
unsafe extern "C" {
    #[wasm_bindgen(js_name = invoke_send_code, catch)]
    pub async fn invoke_send_code() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invoke_get_realtime_game, catch)]
    pub async fn invoke_get_realtime_game() -> Result<JsValue, JsValue>;
}

pub fn get_realtime_game(
    game_data: UseStateHandle<Option<Rc<Realtime>>>,
    counter: Rc<RefCell<usize>>,
) {
    let counter_clone = Rc::clone(&counter);

    spawn_local(async move {
        match invoke_get_realtime_game().await {
            Ok(value) => {
                if let Some(json_string) = value.as_string() {
                    if !json_string.is_empty() {
                        console::log_1(&JsValue::from_str(&format!(
                            "Resposta recebida: {}",
                            json_string
                        )));
                        match serde_json::from_str(&json_string) {
                            Ok(realtime_data) => {
                                console::log_1(&JsValue::from_str("Parsing bem-sucedido"));
                                game_data.set(Some(Rc::new(realtime_data)));
                                *counter_clone.borrow_mut() = 0;
                            }
                            Err(e) => {
                                console::log_1(&JsValue::from_str(&format!(
                                    "Erro de parsing: {}",
                                    e
                                )));
                                *counter_clone.borrow_mut() += 1;
                            }
                        }
                    } else {
                        console::log_1(&JsValue::from_str("Resposta vazia"));
                        *counter_clone.borrow_mut() += 1;
                    }
                } else {
                    console::log_1(&JsValue::from_str("Nenhuma string na resposta"));
                    *counter_clone.borrow_mut() += 1;
                }
            }
            Err(e) => {
                console::log_1(&JsValue::from_str("Erro na requisição"));
                console::log_1(&e);
                *counter_clone.borrow_mut() += 1;
            }
        }
    });
}

pub fn get_code(game_code: UseStateHandle<String>) {
    spawn_local(async move {
        let code = invoke_send_code().await;
        match code {
            Ok(code_value) => {
                if code_value.is_undefined() {
                    console::log_1(&JsValue::from_str("O aplicativo nativo não está em uso"));
                } else {
                    game_code.set(code_value.as_string().unwrap_or_default());
                }
            }
            Err(e) => {
                console::log_1(&e);
            }
        }
    });
}
