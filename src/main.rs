use actix_web::{get, web, Responder, Result};
use element::{Element, Note};
use image::Image;
use serde_json::Value;
use sqlite_image::SqliteImage;

use crate::generic::{Kind, ToJson, UniqueId};

pub mod element;
mod generic;
mod image;
mod sqlite_image;
mod tests;

fn main() {
    let mut img = SqliteImage::new("demo".to_string());

    img.load(image::Mode::File);

    img.add_element(Element::note("Hwllo".to_string(), "Des".to_string()));
    img.add_element(Element::note("Hwllo1".to_string(), "Des".to_string()));
    img.add_element(Element::note("Hwllo2".to_string(), "Des".to_string()));
    img.add_element(Element::note("Hwllo3".to_string(), "Des".to_string()));

    img.close();
}
