mod payment;

use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClientInfo {
    name: Option<String>,
    vatin: Option<String>,
    #[serde(alias = "emailOrPhone")]
    email_or_phone: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AtolOperator {
    name: String,
    vatin: String
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AtolTaxTypes {
    tax_type: String,
    sum: Option<f64>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AtolReceiptItem {
    #[serde(alias = "type")]
    item_type: String,
    name: String,
    price: String,
    quantity: f64,
    amount: i8,
    #[serde(alias = "infoDiscountSum")]
    info_discount_sum: Option<f64>,
    tax: AtolTaxTypes,
    piece: Option<bool>,
    barcode: Option<String>,
    #[serde(alias = "barcodeType")]
    barcode_type: Option<String>,
    total: Option<f64>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AtolPaymentTypes {
    #[serde(alias = "type")]
    payment_type: String,
    sum: f64
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AtolRequestItem {
    #[serde(alias = "type")]
    receipt_type: String,
    electronically: Option<bool>,
    #[serde(alias = "taxationType")]
    taxation_type: String,
    operator: AtolOperator,
    #[serde(alias = "clientInfo")]
    client_info: Option<ClientInfo>,
    items: Vec<AtolRequestItem>,
    payments: Vec<AtolPaymentTypes>,
    total: f64
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AtolReceipt {
    items: Vec<AtolRequestItem>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AtolRequest {
    uuid: String,
    request: AtolReceipt
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Payment {
    sale_type: String,
    sale: i8,
    hash: Option<String>,
    core_return: Option<String>,
    request: AtolRequest
}