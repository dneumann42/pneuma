use actix_web::web;
use image::Image;
use sqlite_image::SqliteImage;

use crate::api::notes::{mutations::add_note, queries::notes};

mod api;
mod element;
mod generic;
mod image;
mod sqlite_image;
mod tests;

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
