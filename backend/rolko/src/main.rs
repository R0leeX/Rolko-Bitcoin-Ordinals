use actix_multipart::{Field, Multipart};
use actix_web::{web, App, HttpResponse, HttpServer, Error};
use futures::{StreamExt, TryStreamExt};
use std::env;
use std::fs::create_dir_all;
use std::path::Path;
use std::fs::File;
use std::io::Write;

async fn upload(mut payload: Multipart) ->  Result<HttpResponse, Error> {
    // Create the uploads directory if it does not exist
    let uploads_dir = env::var("UPLOADS_DIR").unwrap_or_else(|_| String::from("./uploads"));
    create_dir_all(&uploads_dir).unwrap();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();

        // Save file to disk
        let filepath = Path::new(&uploads_dir).join(filename);
        let mut file = web::block(|| File::create(filepath)).await.unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file = web::block(move || file.write_all(&data).map(|_| file)).await?;
        }

        
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/upload", web::post().to(upload)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}