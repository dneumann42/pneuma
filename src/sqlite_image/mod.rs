mod insertables;
mod queries;

use core::panic;

use rusqlite::{params, Connection, Params, Row, Statement};
use uuid::Uuid;

use crate::{
    element::{self, Element, Heading, Note},
    generic::{Insertable, Kind, Named, SqlExec, SqlQuery, Title, UniqueId},
    image::{Image, Mode},
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
        fn query(mut stmt: Statement, id: Uuid) -> Result<String, ()> {
            Ok(stmt
                .query_row(params![id.to_string()], |row| {
                    Ok(row.get::<usize, String>(0)?)
                })
                .unwrap_or("".to_string()))
        }

        self.exec_query(GET_KIND, |s| query(s, id))
            .unwrap_or("".to_string())
    }

    fn to_note(row: &Row) -> Result<Note, rusqlite::Error> {
        Ok(Note::new(row.get(1)?, row.get(2)?))
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
    fn exec_query<R, F>(&self, sql: &str, f: F) -> Result<R, ()>
    where
        R: Default,
        F: FnOnce(Statement) -> Result<R, ()>,
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
    fn load(&mut self, mode: Mode) {
        self.conn = Connection::open(match mode {
            Mode::File => format!("{}.sqlite", self.name()).to_string(),
            Mode::Memory => ":memory:".to_string(),
        })
        .ok();
        self.exec(ELEMENTES_TABLE, ());
        self.exec(NOTES_TABLE, ());
        self.exec(HEADINGS_TABLE, ());
        println!("[TABLES CREATED]");
    }

    fn close(self) {
        let Some(conn) = self.conn else { return };
        let _ = conn.close();
        println!("[CLOSED]");
    }

    fn get_element_kind_by_id(&self, id: Uuid) -> String {
        self.get_kind(id)
    }

    fn get_element_by_id(&self, id: Uuid) -> Option<Element> {
        let kind = self.get_kind(id);
        if kind.as_str() == Note::kind_const().as_str() {
            self.get_note_by_id(id)
                .map(|note| Element::make(id, element::Fragment::Note(note)))
        } else if kind.as_str() == Heading::kind_const().as_str() {
            self.get_note_by_id(id)
                .map(|note| Element::make(id, element::Fragment::Note(note)))
        } else {
            None
        }
    }

    fn get_note_by_id(&self, id: Uuid) -> Option<Note> {
        self.exec_query(GET_NOTE, |mut stmt| {
            Ok(stmt
                .query_row(params![id.to_string()], SqliteImage::to_note)
                .unwrap_or(Default::default()))
        })
        .ok()
    }

    fn get_heading_by_id(&self, id: Uuid) -> Option<Heading> {
        self.exec_query(GET_NOTE, |mut stmt| {
            Ok(stmt
                .query_row(params![id.to_string()], SqliteImage::to_heading)
                .unwrap_or(Default::default()))
        })
        .ok()
    }

    fn add_element(&self, el: Element) -> Uuid {
        el.insert(el.uid(), self);
        el.uid()
    }

    fn exec_query_row<T, F>(&self, sql: &str, f: F) -> Option<T>
    where
        F: FnOnce(&Row) -> Result<T, rusqlite::Error>,
        T: Default,
    {
        todo!()
    }

    fn get_all_notes(&self) -> Vec<Note> {
        match self.exec_query(GET_NOTES, |mut stmt| {
            let x = stmt.query_map([], SqliteImage::to_note);

            match x {
                Ok(xx) => {
                    let mut result: Vec<Note> = vec![];

                    for p in xx {
                        match p {
                            Ok(v) => result.push(v),
                            Err(_) => panic!(""),
                        }
                    }

                    Ok(result)
                }
                Err(e) => panic!("EERRR: {:?}", e),
            }
        }) {
            Ok(xs) => xs,
            Err(_) => vec![],
        }
    }
}
