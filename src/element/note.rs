use serde_json::{json, Value};

use crate::generic::{Descr, Kind, Title, TitleDescr, ToJson};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Note {
    title: String,
    descr: String,
}

impl Note {
    pub fn new(title: String, descr: String) -> Self {
        Self { title, descr }
    }
}

impl Title for Note {
    fn title(&self) -> String {
        self.title.clone()
    }
}

impl Descr for Note {
    fn descr(&self) -> String {
        self.descr.clone()
    }
}

impl TitleDescr for Note {}

impl Kind for Note {
    fn kind(&self) -> String {
        Note::kind_const()
    }
    fn kind_const() -> String {
        String::new()
    }
}

impl ToJson for Note {
    fn to_json(&self) -> Value {
        json!({
            "kind": self.kind(),
            "title": self.title(),
            "descr": self.descr(),
        })
    }
}
