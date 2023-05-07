use actix_web::{web, Responder, Result};
use serde::Deserialize;
use serde_json::json;

use crate::{
    element::element::Element,
    image::{self, Image},
    sqlite_image::SqliteImage,
};

#[derive(Deserialize)]
pub struct AddNoteInput {
    title: String,
    descr: String,
}

pub async fn add_note(info: web::Json<AddNoteInput>) -> Result<impl Responder> {
    let mut img = SqliteImage::new("demo".to_string());
    img.connect(image::Mode::File);
    let id = img
        .add_element(Element::note(&info.title, &info.descr))
        .to_string();
    img.close();
    Ok(web::Json(json!({ "success": id })))
}
