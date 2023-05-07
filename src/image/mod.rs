use rusqlite::{params, Row};
use uuid::Uuid;

use crate::element::{Element, Heading, Note};

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

    fn exec_query_row<T, F>(&self, sql: &str, f: F) -> Option<T>
    where
        F: FnOnce(&Row) -> Result<T, rusqlite::Error>,
        T: Default
    {
        todo!()
    }

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
