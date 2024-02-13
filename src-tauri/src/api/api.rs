use serde_json::{json, Value};
use crate::api::{DocaAPI, DocaRequest};

impl DocaAPI {
    pub fn new(url: &str) -> DocaAPI {
        DocaAPI{
            api_url: url.to_string(),
            jwt_key: "".to_string()
        }
    }
    async fn atol_request(self, command: &str, data: Value ) -> Option<Value> {
        DocaRequest::new(
            "atol",
            command,
            data,
            Option::from(self.jwt_key.clone()))
            .send_request(&self.api_url)
            .await
    }

    pub async fn get_transactions(self, cashbox_id: &str) -> Option<Value> {
        self.atol_request("get-transactions", json!({ "cashbox_id": cashbox_id })).await
    }
    pub async fn get_operations(self, cashbox_id: &str) -> Option<Value> {
        self.atol_request("get-operations", json!({ "cashbox_id": cashbox_id })).await
    }
    pub async fn confirm_transaction(self, sale_id: i64) -> Option<Value> {
        self.atol_request("confirm-transaction", json!({ "sale_id": sale_id })).await
    }
    pub async fn decline_transaction(self, sale_id: i64) -> Option<Value> {
        self.atol_request("decline-transaction", json!({ "sale_id": sale_id })).await
    }
    pub async fn init_jwt(mut self, email: &str, password: &str) -> Result<DocaAPI, String> {
        let response = DocaRequest::new(
            "users",
            "sign-in",
            json!({ "email": email, "password": password }),
            None)
            .send_request(&self.api_url)
            .await;

        if response.is_none() {
            return Err("No jwt key was provided".to_string());
        }

        self.jwt_key = match response {
            Some(value) => value.as_str().unwrap().to_string(),
            None => "".to_string()
        };

        Ok(self)
    }
}