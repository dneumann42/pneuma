use actix_web::{get, web, Responder, Result};
use serde_json::{json, Value};

use crate::generic::ToJson;
use crate::{
    element::element::Element,
    image::{self, Image},
    sqlite_image::SqliteImage,
};

#[get("/notes")]
pub async fn notes() -> Result<impl Responder> {
    let mut img = SqliteImage::new("demo".to_string());
    img.connect(image::Mode::File);
    let notes: Vec<Value> = img.get_all_notes().iter().map(Element::to_json).collect();
    img.close();
    Ok(web::Json(json!(notes)))
}
