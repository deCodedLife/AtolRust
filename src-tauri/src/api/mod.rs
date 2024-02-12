mod jwt;
mod api;

use std::io::Error;
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DocaRequest {
    jwt: Option<String>,
    object: String,
    command: String,
    data: Map<String, Value>,
}

impl DocaRequest {
    fn send_request() -> Result<Value, Error> {
        return Ok(Value::Null)
    }
}

pub struct DocaAPI {}