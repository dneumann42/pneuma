use rusqlite::{types::FromSql, Connection, Params, Row, Statement};

pub trait Conn {
    fn raw(&self) -> &Connection;
    fn exec<T: Into<String>, P: Params>(&self, sql: T, params: P) {
        match self.raw().execute(&sql.into(), params) {
            Ok(ok) => println!("ok {:?}", ok),
            // TODO: handle this
            Err(err) => println!("error {:?}", err),
        }
    }

    fn query<R, F>(&self, sql: &str, f: F) -> Result<R, String>
    where
        R: Default,
        F: FnOnce(Statement) -> Result<R, String>,
    {
        match self.raw().prepare(sql).map(f).map_err(|e| e.to_string()) {
            Ok(Ok(v)) => Ok(v),
            Ok(e) => e,
            Err(e) => Err(e),
        }
    }

    fn get<R>(&self, row: &Row, idx: usize) -> Result<R, String>
    where
        R: FromSql,
    {
        row.get::<usize, R>(idx).map_err(|e| e.to_string())
    }

    fn query_row<R, P: Params, F>(&self, sql: &str, ps: P, f: F) -> Result<R, String>
    where
        R: Default,
        F: FnOnce(&Row) -> Result<R, String>,
    {
        self.raw()
            .prepare(sql)
            .map_err(|e| e.to_string())
            .map(|mut stmt| {
                stmt.query_row(ps, |row| {
                    f(row).map_err(|s| rusqlite::Error::InvalidParameterName(s))
                })
                .map_err(|s| s.to_string())
            })?
    }

    fn query_rows<R, P: Params, F>(&self, sql: &str, ps: P, mut f: F) -> Result<Vec<R>, String>
    where
        F: FnMut(&Row) -> Result<R, String>,
    {
        self.raw()
            .prepare(sql)
            .map_err(|e| e.to_string())
            .map(|mut stmt| {
                match stmt.query_map(ps, |row| {
                    f(row).map_err(|s| rusqlite::Error::InvalidParameterName(s))
                }) {
                    Ok(v) => {
                        let mut vs = vec![];
                        for x in v {
                            vs.push(x.map_err(|e| e.to_string())?);
                        }
                        Ok(vs)
                    }
                    Err(e) => Err(e.to_string()),
                }
            })?
    }
}
