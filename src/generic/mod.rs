use std::{error::Error, fmt::Debug};

use rusqlite::{Params, Row, Statement};
use serde_json::Value;
use uuid::Uuid;

pub trait Title
where
    Self: Sized,
{
    fn title(&self) -> String {
        String::new()
    }
}

pub trait Descr
where
    Self: Sized,
{
    fn descr(&self) -> String {
        String::new()
    }
}

pub trait TitleDescr: Title + Descr {}

pub trait UniqueId<T>
where
    Self: Sized,
    T: Default + ToString,
{
    fn uid(&self) -> T {
        Default::default()
    }

    fn to_string(&self) -> String {
        self.uid().to_string()
    }
}

pub trait Kind {
    fn kind(&self) -> String {
        String::new()
    }

    fn kind_const() -> String {
        String::new()
    }
}

pub trait Named {
    fn name(&self) -> &str {
        ""
    }
}

pub trait SqlExec {
    fn exec<T>(&self, _sql: &str, _params: T)
    where
        T: Params,
    {
    }
}

pub trait SqlQuery {
    fn exec_query<R, F>(&self, _sql: &str, _f: F) -> Result<R, String>
    where
        R: Default,
        F: FnMut(Statement) -> Result<R, String>,
    {
        panic!("not implemented");
    }
}

pub trait Insertable {
    fn insert<S>(&self, _id: Uuid, _sql: &S)
    where
        S: SqlExec,
        Self: Debug,
    {
        panic!("Unimplemented {:?}", self)
    }
}

pub trait Query {
    fn query<S, R>(&self, _id: Uuid, _sql: &S) -> R
    where
        S: SqlExec,
        Self: Debug,
    {
        panic!("Unimplemented {:?}", self)
    }
}

pub trait ToJson {
    fn to_json(&self) -> Value {
        todo!()
    }
    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
}
