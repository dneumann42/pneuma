use serde_json::{json, Value};

use crate::generic::{Kind, ToJson};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Heading {
    HeadingLevel(i32),
}

impl Heading {
    pub fn heading(&self) -> i32 {
        match self {
            Heading::HeadingLevel(n) => n.to_owned(),
        }
    }
}

impl Default for Heading {
    fn default() -> Self {
        Heading::HeadingLevel(1)
    }
}

impl Kind for Heading {
    fn kind(&self) -> String {
        Heading::kind_const()
    }
    fn kind_const() -> String {
        "heading".to_string()
    }
}

impl ToJson for Heading {
    fn to_json(&self) -> Value {
        json!({
            "kind": self.kind(),
            "heading_level": self.heading()
        })
    }

    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
}
