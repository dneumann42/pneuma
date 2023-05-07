use uuid::Uuid;

use crate::{
    element::{element::Element, fragment::Fragment, heading::Heading, note::Note},
    generic::{Descr, Insertable, Kind, SqlExec, Title},
};

const ADD_ELEM: &str = include_str!("../sql/mutations/add_elem.sql");
const ADD_NOTE: &str = include_str!("../sql/mutations/add_note.sql");
const ADD_HEADING: &str = include_str!("../sql/mutations/add_note.sql");

impl Insertable for Note {
    fn insert<S>(&self, id: Uuid, sql: &S)
    where
        S: SqlExec,
    {
        sql.exec(ADD_ELEM, (id.to_string(), self.kind()));
        sql.exec(ADD_NOTE, (id.to_string(), self.title(), self.descr()));
    }
}

impl Insertable for Heading {
    fn insert<S>(&self, id: Uuid, sql: &S)
    where
        S: SqlExec,
    {
        sql.exec(ADD_ELEM, (id.to_string(), self.kind()));
        sql.exec(
            ADD_HEADING,
            (id.to_string(), "note".to_string(), self.heading()),
        );
    }
}

impl Insertable for Fragment {
    fn insert<S>(&self, id: Uuid, sql: &S)
    where
        S: SqlExec,
        Self: std::fmt::Debug,
    {
        match self {
            Fragment::Note(n) => n.insert(id, sql),
            Fragment::Heading(h) => h.insert(id, sql),
        }
    }
}

impl Insertable for Element {
    fn insert<S>(&self, id: Uuid, sql: &S)
    where
        S: SqlExec,
    {
        self.fragment().insert(id, sql);
    }
}
