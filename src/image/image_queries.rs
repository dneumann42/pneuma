use uuid::Uuid;

use crate::element::{element::Element, note::Note};

pub trait ImageQueries
where
    Self: Sized,
{
    fn get_element(&self, _id: Uuid) -> Option<Element> {
        todo!()
    }

    fn get_note(&self, _id: Uuid) -> Option<Note> {
        todo!()
    }

    fn get_all_notes(&self) -> Vec<Element> {
        todo!()
    }
}
