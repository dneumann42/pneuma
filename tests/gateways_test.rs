use pneuma::{
    gateways::{
        tables::create_tables,
        tasks::{
            mutations::{add_task, delete_elem},
            queries::get_tasks,
        },
    },
    sql::Conn,
};
use rusqlite::Connection;

struct SqlTest {
    conn: Connection,
}

impl SqlTest {
    fn open_memory() -> Result<Self, String> {
        let conn = Connection::open_in_memory().map_err(|e| e.to_string())?;
        let this = Self { conn };
        create_tables(&this);
        Ok(this)
    }
}

impl Conn for SqlTest {
    fn raw(&self) -> &Connection {
        &self.conn
    }
}

#[test]
fn it_can_add_and_get_tasks() {
    // let vs = vec![];
    let sql = SqlTest::open_memory().unwrap();
    add_task(&sql, "abc");
    let xs = get_tasks(&sql).unwrap();
    assert_eq!(xs.len(), 1)
}

#[test]
fn it_can_delete_elems() {
    // let vs = vec![];
    let sql = SqlTest::open_memory().unwrap();
    let id = add_task(&sql, "abc");
    let xs = get_tasks(&sql).unwrap();
    assert_eq!(xs.len(), 1);
    delete_elem(&sql, id);
    let xs2 = get_tasks(&sql).unwrap();
    assert_eq!(xs2.len(), 0);
}
