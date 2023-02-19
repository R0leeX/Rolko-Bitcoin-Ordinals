use std::io::Write;

// Actix
use actix_multipart::Multipart;
use actix_web::{web, http, App, Error, error, HttpResponse, HttpServer};
use actix_cors::Cors;
use futures_util::TryStreamExt as _;
use uuid::Uuid;
use serde::Deserialize;

// Bitcoin
use bitcoin::{Address, Network};
use std::str::FromStr;

async fn upload(mut payload: Multipart) ->  Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();
        let filename = content_disposition
        .get_filename()
        .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let filepath = format!("./uploads/{filename}");

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

//TODO: Move this to Bitcoin specific file
fn is_valid_bitcoin_address(address_str: &str) -> bool {
    if let Ok(bitcoin_address) = Address::from_str(address_str) {
        return bitcoin_address.is_valid_for_network(Network::Bitcoin);
    }
    false
}

#[derive(Deserialize)]
struct BitcoinAddress {
    address: String,
}

async fn send_address(data: web::Json<BitcoinAddress>) -> Result<HttpResponse, Error> {
    // Extract the Bitcoin address from the JSON payload
    let bitcoin_address = &data.address;
    if !is_valid_bitcoin_address(&bitcoin_address) {
        return Err(error::ErrorBadRequest("Invalid bitcoin address"));
    }
    // TODO: Send the Bitcoin address

    // Return a response indicating success
    Ok(HttpResponse::Ok().body("Bitcoin address sent successfully"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/api/send-address", web::post().to(send_address))
            .route("/api/upload", web::post().to(upload))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}