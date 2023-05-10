use rusqlite::params;
use uuid::Uuid;

use crate::{
    elements::{Elem, Fragment, Status, Task},
    sql::Conn,
};

const GET_TASKS: &'static str = include_str!("../../sql/queries/get_tasks.sql");

pub fn get_tasks<T: Conn>(sql: &T) -> Result<Vec<Elem>, String> {
    // sql.exec(GET_TASKS, ();
    sql.query_rows(GET_TASKS, params![], |row| {
        let status: String = sql.get(row, 2)?;
        Ok(Elem {
            id: sql.get(row, 0)?,
            fragment: Fragment::Task(Task {
                content: sql.get(row, 1)?,
                status: match status.as_str() {
                    "Incomplete" => Status::Incomplete,
                    "InProgress" => Status::InProgress,
                    "OnHold" => Status::OnHold,
                    "Complete" => Status::Completed,
                    _ => Status::Incomplete,
                },
            }),
        })
    })
}
