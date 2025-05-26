use std::rc::Rc;

use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::model::{
    calculator::{Calculator, GameX},
    realtime::Realtime,
};

#[wasm_bindgen(module = "/public/glue.js")]
unsafe extern "C" {
    #[wasm_bindgen(js_name = invoke_send_code, catch)]
    pub async fn invoke_send_code() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invoke_get_realtime_game, catch)]
    pub async fn invoke_get_realtime_game() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invoke_calculate, catch)]
    pub async fn invoke_calculate(game_state: JsValue) -> Result<JsValue, JsValue>;
}

pub fn get_calculator_result(game_state: GameX, state_handler: UseStateHandle<Option<Calculator>>) {
    spawn_local(async move {
        let game_json = serde_json::to_string(&game_state).unwrap();
        match invoke_calculate(JsValue::from_str(&game_json)).await {
            Ok(value) => {
                console::log_1(&value);
                match serde_json::from_str(&value.as_string().unwrap()) {
                    Ok(calculator_data) => {
                        state_handler.set(Some(calculator_data));
                    }
                    Err(e) => {
                        console::log_1(&JsValue::from_str(e.to_string().as_str()));
                    }
                }
            }
            Err(e) => {
                console::log_1(&e);
            }
        }
    });
}

pub fn get_realtime_game(game_data: UseStateHandle<Option<Rc<Realtime>>>) {
    spawn_local(async move {
        match invoke_get_realtime_game().await {
            Ok(value) => {
                console::log_1(&value);
                let json_string = value.as_string().unwrap_or_default();
                match serde_json::from_str(&json_string) {
                    Ok(realtime_data) => {
                        game_data.set(Some(Rc::new(realtime_data)));
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

pub fn get_code(game_code: UseStateHandle<String>) {
    spawn_local(async move {
        let code = invoke_send_code().await;
        match code {
            Ok(code_value) => {
                if code_value.is_undefined() {
                    console::log_1(&JsValue::from_str("O aplicativo nativo não está em uso"));
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
