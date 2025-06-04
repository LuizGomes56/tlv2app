use std::{cell::RefCell, ops::Deref, rc::Rc};

use gloo::timers::callback::Interval;
use reqwasm::http::Request;
use serde_json::json;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::{
    BACKEND_URL, MAX_FAILURES,
    model::{realtime::Realtime, server::ServerResponse},
};

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
    spawn_local(async move {
        match invoke_get_realtime_game(game_code).await {
            Ok(value) => {
                if let Some(json_string) = value.as_string() {
                    if !json_string.is_empty() {
                        match serde_json::from_str(&json_string) {
                            Ok(realtime_data) => {
                                game_data.set(Some(Rc::new(realtime_data)));
                                *counter.borrow_mut() = 0;
                            }
                            Err(e) => {
                                console::log_1(
                                    &format!("invoke_get_realtime_game parsing error: {}", e)
                                        .into(),
                                );
                                *counter.borrow_mut() = MAX_FAILURES;
                            }
                        }
                    } else {
                        console::log_1(
                            &"Empty string received from Tauri realtime invoker. An unexpected error ocurred".into(),
                        );
                        *counter.borrow_mut() = MAX_FAILURES;
                    }
                } else {
                    console::log_1(
                        &"Empty string received from realtime invoker. Probably user is not using Tauri application".into(),
                    );
                    match Request::post(&format!("{}/api/games/get_by_code", BACKEND_URL))
                        .header("Content-Type", "application/json")
                        .body(
                            json!({
                                "game_code": game_code,
                                "simulated_items": [3115]
                            })
                            .to_string(),
                        )
                        .send()
                        .await
                    {
                        Ok(api_response) => match api_response
                            .json::<ServerResponse<Option<Realtime>>>()
                            .await
                        {
                            Ok(server_response) => {
                                if let Some(realtime_data) = server_response.data {
                                    game_data.set(Some(Rc::new(realtime_data)));
                                    *counter.borrow_mut() = 0;
                                } else {
                                    console::log_1(
                                        &format!(
                                            "Realtime game not found. Message: {}",
                                            server_response
                                                .message
                                                .unwrap_or("Unknown error".to_string())
                                        )
                                        .into(),
                                    );
                                    *counter.borrow_mut() = MAX_FAILURES;
                                }
                            }
                            Err(e) => {
                                console::log_1(&format!("Error while parsing server response at realtime invoker: {:#?}", e).into());
                                *counter.borrow_mut() = MAX_FAILURES;
                            }
                        },
                        Err(e) => {
                            console::log_1(
                                &format!(
                                    "error when trying to get realtime game by code directly: {}",
                                    e
                                )
                                .into(),
                            );
                            *counter.borrow_mut() = MAX_FAILURES;
                        }
                    }
                }
            }
            Err(e) => {
                console::log_1(
                    &format!("unsafe extern 'C' from WASM bindgen failed: {:#?}", e).into(),
                );
                *counter.borrow_mut() = MAX_FAILURES;
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
