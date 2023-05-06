mod insertables;

use rusqlite::{Connection, Params};
use uuid::Uuid;

use crate::{
    element::{Element, Fragment, Heading, Note},
    generic::{Descr, Insertable, Named, SqlExec, Title, UniqueId},
    image::Image,
};

const ELEMENTES_TABLE: &str = include_str!("../sql/tables/elements_table.sql");
const HEADINGS_TABLE: &str = include_str!("../sql/tables/headings_table.sql");
const NOTES_TABLE: &str = include_str!("../sql/tables/notes_table.sql");

pub struct SqliteImage {
    name: String,
    conn: Option<Connection>,
}

impl SqliteImage {
    pub fn new(name: String) -> Self {
        Self { conn: None, name }
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

impl Named for SqliteImage {
    fn name(&self) -> &str {
        &self.name
    }
}

const ADD_ELEM: &str = include_str!("../sql/mutations/add_elem.sql");
const ADD_NOTE: &str = include_str!("../sql/mutations/add_note.sql");
const ADD_HEADING: &str = include_str!("../sql/mutations/add_note.sql");

impl Insertable for Note {
    fn insert<S>(&self, id: Uuid, sql: &S)
    where
        S: SqlExec,
    {
        sql.exec(ADD_ELEM, (id.to_string(), "note".to_string()));
        sql.exec(ADD_NOTE, (id.to_string(), self.title(), self.descr()));
    }
}

impl Insertable for Heading {
    fn insert<S>(&self, id: Uuid, sql: &S)
    where
        S: SqlExec,
    {
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

pub fn connection_string_mut<T>(named: &mut T) -> String
where
    T: Named,
{
    format!("{}.sqlite", named.name()).to_string()
}

impl Image for SqliteImage {
    fn load(&mut self) {
        self.conn = Connection::open(connection_string_mut(self).as_str()).ok();
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

    fn query_element_by_id(&self, _: Uuid) -> Option<crate::element::Element> {
        None
    }

    fn add_element(&self, el: Element) {
        el.insert(el.uid(), self)
    }
}
