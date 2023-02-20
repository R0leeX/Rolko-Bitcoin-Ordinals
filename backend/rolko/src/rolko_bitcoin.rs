

// bitcoin related things
pub mod rolko_bitcoin {
    use bitcoincore_rpc::{Auth, Client, RpcApi};
    use bitcoin::{Address, Network};
    use std::str::FromStr;
    use serde::Serialize;


    // Lightning invoice
    extern crate secp256k1;
    extern crate lightning;
    extern crate lightning_invoice;
    extern crate bitcoin_hashes;
    use bitcoin_hashes::Hash;
    use bitcoin_hashes::sha256;
    use secp256k1::Secp256k1;
    use secp256k1::SecretKey;
    use lightning::ln::PaymentSecret;
    use lightning_invoice::{Currency, InvoiceBuilder};

    #[derive(Serialize)]
    pub struct InvoiceResponse {
        pub lightning_invoice: String,
        pub bitcoin_invoice: String,
    }

    pub fn is_valid_bitcoin_address(address_str: &str) -> bool {
        if let Ok(bitcoin_address) = Address::from_str(address_str) {
            return bitcoin_address.is_valid_for_network(Network::Bitcoin);
        }
        false
    }

    // https://docs.rs/lightning-invoice/0.21.0/lightning_invoice/struct.InvoiceBuilder.html#method.amount_milli_satoshis
    pub fn generate_lightning_invoice() -> String {
        let private_key = SecretKey::from_slice(
            &[
                0xe1, 0x26, 0xf6, 0x8f, 0x7e, 0xaf, 0xcc, 0x8b, 0x74, 0xf5, 0x4d, 0x26, 0x9f,
                0xe2, 0x06, 0xbe, 0x71, 0x50, 0x00, 0xf9, 0x4d, 0xac, 0x06, 0x7d, 0x1c, 0x04,
                0xa8, 0xca, 0x3b, 0x2d, 0xb7, 0x34
            ][..]
        ).unwrap();
        
        let payment_hash = sha256::Hash::from_slice(&[0; 32][..]).unwrap();
        let payment_secret = PaymentSecret([42u8; 32]);
        
        let invoice = InvoiceBuilder::new(Currency::Bitcoin)
            .description("Rolko - Create Ordinals".into())
            .payment_hash(payment_hash)
            .payment_secret(payment_secret)
            .current_timestamp()
            .min_final_cltv_expiry(144)
            .build_signed(|hash| {
                Secp256k1::new().sign_ecdsa_recoverable(hash, &private_key)
            })
            .unwrap();
    
        invoice.to_string()
    }

    pub fn generate_bitcoin_invoice() -> String {
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
}