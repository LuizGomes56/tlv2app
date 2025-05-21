use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotActivePlayer {
    pub riot_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotAllPlayers {
    pub champion_name: String,
    pub riot_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotRealtime {
    pub active_player: RiotActivePlayer,
    pub all_players: Vec<RiotAllPlayers>,
    pub game_data: RiotRealtimeGameData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotRealtimeGameData {
    pub game_time: f64,
}

pub async fn create_game_register(pool: &PgPool, game_code: String, game_id: String) {
    query("INSERT INTO games (game_id, game_code) VALUES ($1, $2)")
        .bind(game_id)
        .bind(game_code)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn commit_game_register(pool: &PgPool, game_id: String) -> Result<String, String> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| format!("Falha ao criar HTTP client: {}", e))?;

    let resp = client
        .get("https://127.0.0.1:2999/liveclientdata/allgamedata")
        .send()
        .await
        .map_err(|e| format!("Erro na requisição: {}", e))?;

    let data: RiotRealtime = resp
        .json()
        .await
        .map_err(|e| format!("Falha ao parsear JSON: {}", e))?;

    let game_data =
        serde_json::to_string(&data).map_err(|e| format!("Erro ao serializar game data: {}", e))?;

    let game_time = data.game_data.game_time;
    let summoner_name = data.active_player.riot_id.clone();
    let champion_name = data
        .all_players
        .into_iter()
        .find(|p| p.riot_id == summoner_name)
        .map(|p| p.champion_name)
        .unwrap_or_default();

    query("UPDATE games SET champion_name = $1, summoner_name = $2 WHERE game_id = $3")
        .bind(champion_name.clone())
        .bind(summoner_name.clone())
        .bind(game_id.clone())
        .execute(pool)
        .await
        .map_err(|e| format!("Erro ao atualizar games: {}", e))?;

    query(
        "INSERT INTO game_data (game_id, game_data, champion_name, game_time, summoner_name)
        VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(game_id)
    .bind(game_data)
    .bind(champion_name)
    .bind(game_time)
    .bind(summoner_name)
    .execute(pool)
    .await
    .map_err(|e| format!("Erro ao inserir game_data: {}", e))?;

    Ok("Dados enviados com sucesso".into())
}
