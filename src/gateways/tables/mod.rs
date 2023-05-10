use crate::sql::Conn;

const CREATE_ELEMENTS_TABLE: &'static str = include_str!("../../sql/tables/elements.sql");

pub fn create_tables<T: Conn>(conn: &T) {
    let _ = conn.exec(CREATE_ELEMENTS_TABLE, ());
}
