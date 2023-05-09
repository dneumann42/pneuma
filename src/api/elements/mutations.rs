use actix_web::{web, Responder, Result};
use serde::Deserialize;
use serde_json::json;

use crate::{
    image::{self, Image},
    sqlite_image::SqliteImage,
};

#[derive(Deserialize)]
pub struct DeleteElementInput {
    noteId: String,
}

pub async fn delete_element(info: web::Json<DeleteElementInput>) -> Result<impl Responder> {
    let mut img = SqliteImage::new("demo".to_string());
    img.connect(image::Mode::File);



    img.close();
    Ok(web::Json(json!({ "success": true })))
}
