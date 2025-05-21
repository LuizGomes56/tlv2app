use dotenvy::dotenv;
use rand::Rng;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{
    env,
    sync::{Arc, Mutex},
};
use tauri::{Manager, State};
use uuid::Uuid;
mod database;
use database::db::commit_game_register;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    game_code: Arc<Mutex<String>>,
    game_id: Arc<Mutex<String>>,
}

#[tauri::command]
fn send_code(state: State<'_, Arc<AppState>>) -> String {
    state.game_code.lock().unwrap().clone()
}

#[tauri::command]
async fn start_game(state: State<'_, Arc<AppState>>) -> Result<String, String> {
    let pool = state.pool.clone();
    let game_id = state.game_id.lock().unwrap().clone();

    commit_game_register(&pool, game_id).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_code, start_game])
        .setup(|app| {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido");

            let pool = tauri::async_runtime::block_on(async {
                PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&database_url)
                    .await
                    .expect("Falha ao conectar no banco")
            });

            let mut rng = rand::rng();
            let code = format!("{:06}", rng.random_range(0..1_000_000));
            let game_id = Uuid::new_v4().to_string();

            let state = Arc::new(AppState {
                pool: pool.clone(),
                game_code: Arc::new(Mutex::new(code.clone())),
                game_id: Arc::new(Mutex::new(game_id.clone())),
            });

            tauri::async_runtime::block_on(async {
                database::db::create_game_register(&state.pool, code, game_id).await;
            });

            app.manage(state);

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
