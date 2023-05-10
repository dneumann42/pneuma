use pneuma::{gateways::tables::create_tables, sqlconnection::SqlConn};

pub mod elements;
pub mod gateways;
pub mod generic;
pub mod sql;
pub mod sqlconnection;

fn main() {
    let sql = SqlConn::open().unwrap();
    create_tables(&sql);
    sql.close()
}
