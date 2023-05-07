use std::fmt::format;

use actix_web::{get, web, Responder, Result};
use element::element::Element;
use image::Image;
use serde::Deserialize;
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
    img.connect(image::Mode::File);
    let notes: Vec<Value> = img.get_all_notes().iter().map(Element::to_json).collect();
    img.close();
    Ok(web::Json(json!(notes)))
}

#[derive(Deserialize)]
struct AddNoteInput {
    title: String,
    descr: String,
}

async fn add_note(info: web::Json<AddNoteInput>) -> Result<impl Responder> {
    let mut img = SqliteImage::new("demo".to_string());
    img.connect(image::Mode::File);
    let id = img
        .add_element(Element::note(&info.title, &info.descr))
        .to_string();
    img.close();
    Ok(web::Json(json!({ "success": id })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Build tables
    let mut img = SqliteImage::new("demo".to_string());
    img.start(image::Mode::File);
    img.close();

    use actix_web::{App, HttpServer};
    HttpServer::new(|| {
        App::new()
            .service(notes)
            .route("/post/add-note", web::post().to(add_note))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
