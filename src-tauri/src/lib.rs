#![allow(unstable_features)]
#![feature(thread_id_value)]

use std::sync::Arc;

use crate::wnd_system::keyboard::{install_hook, uninstall_hook};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use tauri::{async_runtime, Manager, State};
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, WM_QUIT, WM_USER};

mod model;
mod wnd_system;

#[derive(Deserialize)]
struct ServerResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

struct AppState {
    client: Arc<Client>,
    static_game_code: usize,
    static_game_id: String,
}

#[tauri::command]
fn get_game_code(state: State<'_, AppState>) -> usize {
    state.static_game_code
}

const BACKEND_URL: &str = "http://localhost:8082";

#[tauri::command]
async fn get_realtime_game(state: State<'_, AppState>, game_code: usize) -> Result<String, String> {
    let get_game_data = async |url: &str, json_body: Value| -> Result<String, String> {
        let server_response = state
            .client
            .post(url)
            .json(&json_body)
            .send()
            .await
            .map_err(|e| {
                let error_kind = format!("Error om server response: {:#?}", e);
                println!("{}", error_kind);
                error_kind
            })?;

        let json_response = server_response
            .json::<ServerResponse<Value>>()
            .await
            .map_err(|e| {
                let error_kind = format!("Error ocurred on json parsing: {:#?}", e);
                println!("{}", error_kind);
                error_kind
            })?;

        if let Some(data) = json_response.data {
            return Ok(data.to_string());
        } else if let Some(message) = json_response.message {
            return Err(message);
        } else {
            return Err("Unknown error".to_string());
        }
    };

    if game_code == state.static_game_code {
        println!("Getting local game data because code matches the stored in the app");
        let local_response = state
            .client
            .get("https://127.0.0.1:2999/liveclientdata/allgamedata")
            .send()
            .await
            .map_err(|e| {
                let error_kind = format!("Error ocurred in getting live game: {:#?}", e);
                println!("{}", error_kind);
                error_kind
            })?;

        let game_data = local_response.text().await.unwrap_or_default();

        get_game_data(
            &format!("{}/api/games/realtime", BACKEND_URL),
            json!({
                "game_id": state.static_game_id,
                "game_code": state.static_game_code,
                "game_data": game_data,
                "simulated_items": [3115],
            }),
        )
        .await
    } else {
        println!(
            "Getting a previous game using code because it doesn't match the stored in the app"
        );
        get_game_data(
            &format!("{}/api/games/get_by_code", BACKEND_URL),
            json!({
                "game_code": game_code,
                "simulated_items": [3115],
            }),
        )
        .await
    }
}

#[derive(Deserialize)]
struct CreateGameResponse {
    game_code: usize,
    game_id: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .device_event_filter(tauri::DeviceEventFilter::Always)
        .invoke_handler(tauri::generate_handler![get_realtime_game, get_game_code])
        .setup(|app| {
            #[cfg(target_os = "windows")]
            unsafe {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    install_hook();
                    let mut msg = MSG::default();
                    while GetMessageW(&mut msg, None, 0, 0).into() {
                        if msg.message == WM_QUIT {
                            break;
                        }
                        if msg.message == WM_USER + 1 {
                            if let Some(win) = app_handle.get_webview_window("main") {
                                if !win.is_visible().unwrap() {
                                    let _ = win.show();
                                    let _ = win.set_focus();
                                } else {
                                    let _ = win.hide();
                                }
                            }
                        }
                    }
                    uninstall_hook();
                });
            }
            let client = Client::builder()
                .danger_accept_invalid_certs(true)
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap();

            let initialization: Option<CreateGameResponse> = async_runtime::block_on(async {
                let ServerResponse::<CreateGameResponse> {
                    success,
                    data,
                    message,
                } = client
                    .get(&format!("{}/api/games/create", BACKEND_URL))
                    .send()
                    .await
                    .map_err(|e| println!("Failed request send: {:#?}", e))
                    .ok()?
                    .json()
                    .await
                    .map_err(|e| println!("Json parse error: {:#?}", e))
                    .ok()?;
                if success {
                    data
                } else {
                    println!("Failed to initialize app game creation: {:#?}", message);
                    None
                }
            });

            let (static_game_code, static_game_id) = initialization
                .map(|res| (res.game_code, res.game_id))
                .unwrap_or_default();

            app.manage(AppState {
                client: Arc::new(client),
                static_game_code,
                static_game_id,
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
