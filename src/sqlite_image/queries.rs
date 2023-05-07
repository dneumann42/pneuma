use crate::{
    element::{Element, Fragment, Heading, Note},
    generic::{Descr, Kind, Query, SqlExec, Title},
};

const GET_NOTE: &str = include_str!("../sql/queries/get_note.sql");
const GET_NOTES: &str = include_str!("../sql/queries/get_notes.sql");

impl Query for Note {
    fn query<S, R>(&self, _id: uuid::Uuid, _sql: &S) -> R
    where
        S: SqlExec,
        Self: std::fmt::Debug,
    {
        panic!("Unimplemented {:?}", self)
    }
}
