use pneuma::{gateways::elements::get_kind, gateways::tables::create_tables, sql::Conn};
use rusqlite::Connection;

use crate::{
    elements::Elem,
    generic::json::{FromJson, ToJson},
};

pub mod elements;
pub mod gateways;
pub mod generic;
pub mod sql;

struct SqlConn {
    conn: Connection,
}

impl SqlConn {
    fn open() -> Result<Self, String> {
        let conn = Connection::open("demo.sqlite").map_err(|e| e.to_string())?;
        Ok(Self { conn })
    }

    fn open_memory() -> Result<Self, String> {
        let conn = Connection::open_in_memory().map_err(|e| e.to_string())?;
        Ok(Self { conn })
    }

    fn close(self) {
        let _ = self.conn.close();
    }
}

impl Conn for SqlConn {
    fn raw(&self) -> &Connection {
        &self.conn
    }

    fn exec<T: Into<String>, P: rusqlite::Params>(&self, sql: T, params: P) {
        match self.raw().execute(&sql.into(), params) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

fn main() {
    let sql = SqlConn::open().unwrap();
    // let k = get_kind(&sql, "el".to_string());
    create_tables(&sql);
    sql.close()
}
