use serde_derive::{Deserialize, Serialize};
use serde_json::value;
use uuid::Uuid;

use crate::generic::{
    info::{Descr, Title, TitleDescr, UniqueId},
    json::{FromJson, ToJson},
};

type EId = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Incomplete,
    InProgress,
    OnHold,
    Completed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    content: String,
    status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    title: String,
    descr: String,
    elems: Vec<EId>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Fragment {
    Note(Note),
    Task(Task),
    Doc(Document),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Elem {
    id: EId,
    fragment: Fragment,
}

impl Elem {
    pub fn make() -> Elem {
        Elem {
            id: Uuid::new_v4().to_string(),
            fragment: Fragment::Note(Note {
                content: "".to_string(),
            }),
        }
    }

    pub fn note<T: Into<String>>(content: T) -> Elem {
        Elem {
            id: Uuid::new_v4().to_string(),
            fragment: Fragment::Note(Note {
                content: content.into(),
            }),
        }
    }
}

impl UniqueId for Elem {
    fn uid(&self) -> String {
        self.id.to_string()
    }
}

impl TitleDescr for Document {}

impl Title for Document {
    fn title(&self) -> &String {
        &self.title
    }
}

impl Descr for Document {
    fn descr(&self) -> &String {
        &self.descr
    }
}

impl ToJson for Elem {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or("".to_string())
    }
}

impl FromJson for Elem {
    fn from_json(v: String) -> Result<Self, String> {
        serde_json::from_str(&v).map_err(|err| err.to_string())
    }
}
