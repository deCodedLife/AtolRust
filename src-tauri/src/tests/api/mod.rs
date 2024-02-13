pub use crate::api::{DocaRequest};
use crate::tests::api::expected::{CORRECT_REQUESTS};
use crate::tests::api::unexpected::INCORRECT_REQUESTS;
use tokio;
use crate::api::DocaAPI;

mod expected;
mod unexpected;


const API_URL: &str = "https://demo.docacrm.com/";

#[test]
fn parse_incorrect_requests() {
    for request in INCORRECT_REQUESTS {
        assert_eq!(false, serde_json::from_str::<DocaRequest>(request).is_ok());
    }
}

#[test]
fn parse_correct_requests() {
    for request in CORRECT_REQUESTS {
        serde_json::from_str::<DocaRequest>(request).unwrap();
    }
}

#[tokio::test]
async fn auth_default() {
    DocaAPI::new( API_URL )
        .init_jwt("support@oxbox.ru", "IDEqRe1X4tPQDubh")
        .await
        .unwrap();
}

#[tokio::test]
async fn get_transactions() {
    let _ = DocaAPI::new( API_URL )
        .init_jwt("support@oxbox.ru", "IDEqRe1X4tPQDubh")
        .await
        .unwrap()
        .get_transactions( "1_yashlek" )
        .await;
}