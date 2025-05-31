use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerResponse<T> {
    pub data: T,
}
