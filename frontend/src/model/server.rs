use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: T,
}
