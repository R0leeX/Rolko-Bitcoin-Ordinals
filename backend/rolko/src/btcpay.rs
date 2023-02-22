


use std::collections::HashMap;
use serde::Serialize;

use crate::common::get_config;

#[derive(Serialize)]
pub struct InvoiceResponse {
    pub lightning_invoice: String,
    pub bitcoin_invoice: String,
}


// https://docs.rs/awc/latest/awc/index.html
pub async fn generate_bitcoin_invoice() -> Result<String, reqwest::Error> {
    
    let _store_id = get_config().get_string("store_id");

    let mut json_body = HashMap::new();
    json_body.insert("lang", "rust");
    json_body.insert("body", "json");

    let client = reqwest::Client::new(); // Use ClientBuilder for proxy. https://docs.rs/reqwest/latest/reqwest/#proxies
    let response = client.post("localhost:3003/api/v1/stores/{store_id}/invoices")
        .json(&json_body)
        .send()
        .await?;

    println!("Response: {:?}", response);
    Ok("".to_string())
}

// https://docs.rs/lightning-invoice/0.21.0/lightning_invoice/struct.InvoiceBuilder.html#method.amount_milli_satoshis
pub async fn generate_lightning_invoice() -> Result<String, reqwest::Error> {

    Ok("".to_string())
}
