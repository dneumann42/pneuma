use crate::{
    elements::{EId, Elem, Fragment},
    generic::info::UniqueId,
    sql::Conn,
};

const ADD_ELEM_MUTATION: &'static str = include_str!("../../sql/mutations/add_elem.sql");
const ADD_TASK_MUTATION: &'static str = include_str!("../../sql/mutations/add_task.sql");

const DELETE_ELEM_MUTATION: &'static str = include_str!("../../sql/mutations/delete_elem.sql");

pub fn add_task<T: Conn, S: Into<String>>(sql: &T, content: S) -> EId {
    let task = Elem::task(content);

    match task.frag() {
        Fragment::Task(t) => {
            sql.exec(ADD_ELEM_MUTATION, (task.uid(), "task"));
            sql.exec(
                ADD_TASK_MUTATION,
                (task.uid(), t.content.to_string(), format!("{:?}", t.status)),
            );
            task.id
        }
        _ => task.id,
    }
}

pub fn delete_elem<T: Conn>(sql: &T, eid: EId) {
    sql.exec(DELETE_ELEM_MUTATION, (eid,));
}
