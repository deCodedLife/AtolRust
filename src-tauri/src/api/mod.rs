pub mod api;
mod request;

use serde::{Serialize, Deserialize};
use serde_json::{Value};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DocaRequest {
    jwt: Option<String>,
    object: String,
    command: String,
    data: Value,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DocaResponse {
    pub status: i64,
    pub data: Value,
    pub detail: Option<Vec<Value>>
}

#[derive(Debug, Default)]
pub struct DocaAPI {
    api_url: String,
    jwt_key: String
}