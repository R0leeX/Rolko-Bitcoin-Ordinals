// bitcoin related things
use bitcoin::{Address, Network};
use std::str::FromStr;
use serde::Serialize;


#[derive(Serialize)]
pub struct InvoiceResponse {
    pub lightning_invoice: String,
    pub bitcoin_invoice: String,
}

pub fn is_valid_bitcoin_address(address_str: &str) -> bool {
    if let Ok(bitcoin_address) = Address::from_str(address_str) {
        return bitcoin_address.is_valid_for_network(Network::Bitcoin); // TODO: Also check if address is Taproot address
    }
    false
}

// https://docs.rs/lightning-invoice/0.21.0/lightning_invoice/struct.InvoiceBuilder.html#method.amount_milli_satoshis
pub fn generate_lightning_invoice() -> String {

    "".to_string()
}

pub fn generate_bitcoin_invoice() -> String {

    //btcpay::generate_bitcoin_invoice()


    /* The following will return a bitcoin address but creating invoice is not possible with bitcoin core.
    // Create a client to connect to the Bitcoin Core node
    let auth = Auth::UserPass("username".to_string(), "password".to_string()); // TODO: Load this from config
    let client = Client::new("http://127.0.0.1:8332", auth).unwrap();

    // Set the payment amount in satoshis
    let satoshis = 10000;

    // Generate a new Bitcoin address for receiving the payment
    let address = client.get_new_address(None, None).unwrap();

    println!("{}", address); // TODO: Use logging
    address.to_string()*/
    "".to_string()
}

pub fn process_bitcoin_address(bitcoin_address: &str) -> bool {

    false
}

pub fn monitor_lightning_invoice(lightning_invoice: &str) 
{
    // https://github.com/Kixunil/tonic_lnd/blob/master/examples/subscribe_invoices.rs
}

pub fn monitor_bitcoin_invoice(bitcoin_invoice: &str)
{
    // Check bitcoin_rpc how to handle
}
