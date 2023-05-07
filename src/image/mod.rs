use rusqlite::{params, Row};
use uuid::Uuid;

use crate::element::{element::Element, heading::Heading, note::Note};

pub enum Mode {
    File,
    Memory,
}

pub trait Image
where
    Self: Sized,
{
    fn load(&mut self, mode: Mode) {}
    fn close(self) {}

    fn get_element_by_id(&self, _: Uuid) -> Option<Element> {
        todo!()
    }

    fn get_note_by_id(&self, id: Uuid) -> Option<Note> {
        todo!()
    }

    fn get_all_notes(&self) -> Vec<Note> {
        todo!()
    }

    fn get_heading_by_id(&self, id: Uuid) -> Option<Heading> {
        todo!()
    }

    fn get_element_kind_by_id(&self, id: Uuid) -> String {
        todo!()
    }

    fn add_element(&self, _: Element) -> Uuid {
        todo!()
    }
}
