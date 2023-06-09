mod insertables;

use core::panic;
use std::str::FromStr;

use rusqlite::{params, Connection, MappedRows, Params, Row, Statement};
use uuid::Uuid;

use crate::{
    element::{element::Element, fragment::Fragment, heading::Heading, note::Note},
    generic::{Insertable, Kind, Named, SqlExec, SqlQuery, Title, UniqueId},
    image::{
        image_mutations::{ImageAdds, ImageDeletes, ImageMutations},
        image_queries::ImageQueries,
        Image, Mode,
    },
};

const ELEMENTES_TABLE: &str = include_str!("../sql/tables/elements_table.sql");
const HEADINGS_TABLE: &str = include_str!("../sql/tables/headings_table.sql");
const NOTES_TABLE: &str = include_str!("../sql/tables/notes_table.sql");

const GET_KIND: &str = include_str!("../sql/queries/get_kind.sql");
const GET_NOTE: &str = include_str!("../sql/queries/get_note.sql");
const GET_NOTES: &str = include_str!("../sql/queries/get_notes.sql");

pub struct SqliteImage {
    name: String,
    conn: Option<Connection>,
}

impl SqliteImage {
    pub fn new(name: String) -> Self {
        Self { conn: None, name }
    }

    pub fn get_kind(&self, id: Uuid) -> String {
        fn query(mut stmt: Statement, id: Uuid) -> Result<String, String> {
            match stmt.query_row(params![id.to_string()], |row| {
                Ok(row.get::<usize, String>(0)?)
            }) {
                Ok(ok) => Ok(ok),
                Err(err) => Err(err.to_string().to_owned()),
            }
        }

        self.exec_query(GET_KIND, |s| query(s, id))
            .unwrap_or("".to_string())
    }

    fn to_note(row: &Row) -> Result<Note, rusqlite::Error> {
        Ok(Note::new(&row.get(1)?, &row.get(2)?))
    }

    fn to_note_element(row: &Row) -> Result<Element, rusqlite::Error> {
        let id: String = row.get(0)?;
        Ok(Element::make(
            Uuid::from_str(id.as_str()).unwrap_or(Uuid::new_v4()),
            Fragment::Note(Note::new(&row.get(1)?, &row.get(2)?)),
        ))
    }

    fn to_heading(row: &Row) -> Result<Heading, rusqlite::Error> {
        Ok(Heading::HeadingLevel(row.get(1)?))
    }
}

impl SqlExec for SqliteImage {
    fn exec<T>(&self, sql: &str, params: T)
    where
        T: Params,
    {
        let Some(conn) = &self.conn else { return };
        match conn.execute(sql, params) {
            Ok(ok) => println!("{}", ok),
            Err(err) => {
                panic!("HERE {:?}\nSQL:\n{:?}", err, sql)
            }
        }
    }
}

impl SqlQuery for SqliteImage {
    fn exec_query<R, F>(&self, sql: &str, f: F) -> Result<R, String>
    where
        R: Default,
        F: FnOnce(Statement) -> Result<R, String>,
    {
        if let Some(conn) = &self.conn {
            match conn.prepare(sql) {
                Ok(stmt) => f(stmt),
                Err(_) => todo!(),
            }
        } else {
            todo!()
        }
    }
}

impl Named for SqliteImage {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Image for SqliteImage {
    fn connect(&mut self, mode: Mode) {
        self.conn = Connection::open(match mode {
            Mode::File => format!("{}.sqlite", self.name()).to_string(),
            Mode::Memory => ":memory:".to_string(),
        })
        .ok()
    }

    fn create(&mut self) {
        self.exec(ELEMENTES_TABLE, ());
        self.exec(NOTES_TABLE, ());
        self.exec(HEADINGS_TABLE, ());
    }

    fn start(&mut self, mode: Mode) {
        self.connect(mode);
        self.create();
    }

    fn close(self) {
        let Some(conn) = self.conn else { return };
        let _ = conn.close();
        println!("[CLOSED]");
    }
}

// Mutations

impl ImageMutations for SqliteImage {}

impl ImageAdds for SqliteImage {
    fn add_element(&self, el: Element) -> Uuid {
        el.insert(el.uid(), self);
        el.uid()
    }
}

impl ImageDeletes for SqliteImage {
    fn delete_element(&self, _id: Uuid) {
        todo!()
    }
}

// Queries

impl ImageQueries for SqliteImage {
    fn get_element(&self, id: Uuid) -> Option<Element> {
        let kind = self.get_kind(id);
        if kind.as_str() == Note::kind_const().as_str() {
            self.get_note(id)
                .map(|note| Element::make(id, crate::element::fragment::Fragment::Note(note)))
        } else if kind.as_str() == Heading::kind_const().as_str() {
            self.get_note(id)
                .map(|note| Element::make(id, crate::element::fragment::Fragment::Note(note)))
        } else {
            None
        }
    }

    fn get_note(&self, id: Uuid) -> Option<Note> {
        self.exec_query(GET_NOTE, |mut stmt| {
            Ok(stmt
                .query_row(params![id.to_string()], SqliteImage::to_note)
                .unwrap_or(Default::default()))
        })
        .ok()
    }

    fn get_all_notes(&self) -> Vec<Element> {
        match self.exec_query(GET_NOTES, |mut stmt| {
            match stmt.query_map([], SqliteImage::to_note_element) {
                Ok(mapped_rows) => {
                    let result: Vec<Element> = mapped_rows
                        .into_iter()
                        .map(|p| match p {
                            Ok(v) => v,
                            Err(_) => todo!(),
                        })
                        .collect();
                    Ok(result)
                }
                Err(e) => panic!("Error: {:?}", e),
            }
        }) {
            Ok(xs) => xs,
            Err(_) => vec![],
        }
    }
}
