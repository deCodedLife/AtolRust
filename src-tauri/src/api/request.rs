use serde_json::Value;
use crate::api::{DocaRequest, DocaResponse};

impl DocaRequest {
    // TODO: Add logger
    pub async fn send_request(&self, api_url: &str) -> Option<Value> {
        let post_reqeust = reqwest::Client::new().post( api_url )
            .json(&self)
            .send()
            .await;

        let post_response = match post_reqeust {
            Ok(resp) => resp.json::<DocaResponse>().await,
            Err(_) => return None
        };

        let response = match post_response {
            Ok(body) => body,
            Err(_) => return None
        };

        if response.status != 200 {
            return None
        }

        Option::from(response.data)
    }
    pub fn new (object: &str, command: &str, data: Value, jwt: Option<String> ) -> DocaRequest {
        DocaRequest {
            object: object.to_string(),
            command: command.to_string(),
            data,
            jwt
        }
    }
}