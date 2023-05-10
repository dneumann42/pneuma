use rusqlite::params;

use crate::{elements::EId, sql::Conn};

const GET_KIND: &'static str = include_str!("../../sql/queries/elements/get_kind.sql");

pub fn get_kind<T: Conn>(conn: &T, el: EId) -> Result<String, String> {
    conn.query_row(GET_KIND, params![el.to_string()], |row| {
        Ok(conn.get::<String>(row, 1)?)
    })
}
