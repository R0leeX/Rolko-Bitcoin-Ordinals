


use actix_rt::System;
use actix_web::client::Client;
let store_id = ;

pub fn generate_bitcoin_invoice() -> String {
    
    System::new("test").block_on(lazy(|| {
        let mut client = Client::default();
    
        client.post("localhost:3003/api/v1/stores/{store_id}/invoices") // <- Create request builder
            .header("User-Agent", "Actix-web")
            .send()                             // <- Send http request
            .map_err(|_| ())
            .and_then(|response| {              // <- server http response
                println!("Response: {:?}", response);
                Ok(())
            })
        }));
}