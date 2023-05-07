use actix_web::{get, web, Responder, Result};
use image::Image;
use serde_json::{json, Value};
use sqlite_image::SqliteImage;

use crate::{
    element::note::Note,
    generic::{Kind, ToJson, UniqueId},
};

mod element;
mod generic;
mod image;
mod sqlite_image;
mod tests;

#[get("/notes")]
async fn notes() -> Result<impl Responder> {
    let mut img = SqliteImage::new("demo".to_string());
    img.load(image::Mode::File);

    let notes: Vec<Value> = img.get_all_notes().iter().map(Note::to_json).collect();
    Ok(web::Json(json!(notes)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    HttpServer::new(|| App::new().service(notes))
        .bind(("127.0.0.1", 4000))?
        .run()
        .await
}
