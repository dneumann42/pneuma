pub mod image_mutations;
pub mod image_queries;

use uuid::Uuid;

use crate::element::{element::Element, heading::Heading, note::Note};

use self::{image_mutations::ImageMutations, image_queries::ImageQueries};

pub enum Mode {
    File,
    Memory,
}

pub trait Image: ImageQueries + ImageMutations {
    fn connect(&mut self, _mode: Mode) {}
    fn create(&mut self) {}
    fn start(&mut self, _mode: Mode) {}
    fn close(self) {}
}