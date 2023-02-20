use std::io::Write;

// Actix
use actix_multipart::Multipart;
use actix_web::{web, http, App, Error, error, HttpResponse, HttpServer};
use actix_cors::Cors;
use futures_util::TryStreamExt as _;
use uuid::Uuid;
use serde::Deserialize;

// Bitcoin
pub mod rolko_bitcoin;
use crate::rolko_bitcoin::rolko_bitcoin::*;

async fn upload(mut payload: Multipart) ->  Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();
        
        let filename = content_disposition
        .get_filename()
        .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let filepath = format!("./uploads/{filename}");
        println!("upload 3");
        println!("{}", filepath);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;
        println!("upload 4");
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            println!("upload 5");
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    // TODO:
    // let fee = estimate_fee() // Economy (Default), Normal, Custom
    // fee.network_fee; fee.service_fee -> 25k Satoshi + 10% of network fee

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

#[derive(Deserialize)]
struct BitcoinAddress {
    address: String,
}
async fn send_address(data: web::Json<BitcoinAddress>) -> Result<HttpResponse, Error> {
    let bitcoin_address = &data.address;

    if !process_bitcoin_address(&bitcoin_address) {
        return Err(error::ErrorBadRequest("Processing request failed"));
    } else {
         // Return a response indicating success
    Ok(HttpResponse::Ok().body("Bitcoin address processed successfully"))
    }

}

// Workflow:
// validate_address -> generate_invoice -> monitor if invoice is paid -> If invoice is paid -> process_ordinals
async fn validate_address(data: web::Json<BitcoinAddress>) -> Result<HttpResponse, Error> {
    // Extract the Bitcoin address from the JSON payload
    let bitcoin_address = &data.address;
    if !is_valid_bitcoin_address(&bitcoin_address) {
        return Err(error::ErrorBadRequest("Invalid bitcoin address"));
    } else {
        // Return a response indicating success
        Ok(HttpResponse::Ok().body("Bitcoin address sent successfully"))
    }
}



async fn generate_invoices(data: web::Json<BitcoinAddress>) -> Result<actix_web::web::Json<InvoiceResponse>, Error> {
   
    let invoice_resp = InvoiceResponse {
        lightning_invoice : generate_lightning_invoice(),
        bitcoin_invoice : generate_bitcoin_invoice(),
    };

    monitor_lightning_invoice(&invoice_resp.lightning_invoice); 
    monitor_bitcoin_invoice(&invoice_resp.bitcoin_invoice); // maybe monitor bitcoin wallet is better

    Ok(web::Json(invoice_resp))
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
            .route("/api/validate-address", web::post().to(validate_address))
            .route("/api/upload", web::post().to(upload))
            .route("/api/generate-invoices", web::post().to(generate_invoices))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}