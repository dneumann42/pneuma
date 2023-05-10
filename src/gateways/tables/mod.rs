use crate::sql::Conn;

const CREATE_ELEMENTS_TABLE: &'static str = include_str!("../../sql/tables/elements.sql");
const CREATE_TASKS_TABLE: &'static str = include_str!("../../sql/tables/tasks.sql");

pub fn create_tables<T: Conn>(conn: &T) {
    println!("{:?}", conn.exec(CREATE_ELEMENTS_TABLE, ()));
    println!("{:?}", conn.exec(CREATE_TASKS_TABLE, ()));
}
