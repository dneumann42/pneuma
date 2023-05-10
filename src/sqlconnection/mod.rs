use rusqlite::Connection;

use crate::sql::Conn;

pub struct SqlConn {
    conn: Connection,
}

impl SqlConn {
    pub fn open() -> Result<Self, String> {
        let conn = Connection::open("demo.sqlite").map_err(|e| e.to_string())?;
        Ok(Self { conn })
    }

    pub fn open_memory() -> Result<Self, String> {
        let conn = Connection::open_in_memory().map_err(|e| e.to_string())?;
        Ok(Self { conn })
    }

    pub fn close(self) {
        let _ = self.conn.close();
    }
}

impl Conn for SqlConn {
    fn raw(&self) -> &Connection {
        &self.conn
    }
}
