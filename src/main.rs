use element::{Element, Note};
use image::Image;
use sqlite_image::SqliteImage;

mod element;
mod generic;
mod image;
mod sqlite_image;

fn main() {
    let mut img = SqliteImage::new("demo".to_string());

    img.load();

    // let el = Element::new

    img.add_element(Element::new(element::Fragment::Note(Note {
        title: "Hello".to_owned(),
        descr: "World".to_owned(),
    })));

    img.add_element(Element::new(element::Fragment::Note(Note {
        title: "Hello".to_owned(),
        descr: "World".to_owned(),
    })));

    img.close();
}
