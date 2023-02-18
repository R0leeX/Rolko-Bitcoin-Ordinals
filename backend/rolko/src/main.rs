use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures_util::TryStreamExt as _;
use uuid::Uuid;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/api/upload", web::post().to(upload)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}