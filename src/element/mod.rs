use serde_json::{json, Value};
use uuid::Uuid;

use crate::generic::{Descr, Kind, Title, TitleDescr, ToJson, UniqueId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Heading {
    HeadingLevel(i32),
}

impl Default for Heading {
    fn default() -> Self {
        Heading::HeadingLevel(1)
    }
}

impl Heading {
    pub fn heading(&self) -> i32 {
        match self {
            Heading::HeadingLevel(n) => n.to_owned(),
        }
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
        self.descr
    }
}

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

impl TitleDescr for Note {}

#[derive(Clone, Debug, PartialEq)]
pub enum Fragment {
    Note(Note),
    Heading(Heading),
}

impl ToJson for Fragment {
    fn to_json(&self) -> Value {
        match self {
            Fragment::Note(n) => n.to_json(),
            Fragment::Heading(h) => h.to_json(),
        }
    }
}

impl Kind for Fragment {
    fn kind(&self) -> String {
        match self {
            Fragment::Note(_) => Note::kind_const(),
            Fragment::Heading(_) => Heading::kind_const(),
        }
        .to_owned()
    }

    fn kind_const() -> String {
        String::from("fragment")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub id: Uuid,
    pub fragment: Fragment,
}

impl Kind for Element {
    fn kind(&self) -> String {
        self.fragment.kind()
    }
}

impl Element {
    pub fn new(fragment: Fragment) -> Self {
        Element {
            id: Uuid::new_v4(),
            fragment,
        }
    }

    pub fn make(id: Uuid, fragment: Fragment) -> Element {
        Element { id, fragment }
    }

    pub fn fragment(&self) -> &Fragment {
        &self.fragment
    }

    pub fn note(title: String, descr: String) -> Self {
        Self::new(Fragment::Note(Note { title, descr }))
    }

    pub fn heading(heading_level: i32) -> Self {
        Self::new(Fragment::Heading(Heading::HeadingLevel(heading_level)))
    }
}

impl ToJson for Element {
    fn to_json(&self) -> Value {
        json!({
            "type": "Element",
            "id": self.uid().to_string(),
            "fragment": self.fragment.to_json()
        })
    }
}

impl UniqueId<Uuid> for Element {
    fn uid(&self) -> Uuid {
        self.id
    }

    fn to_string(&self) -> String {
        self.uid().to_string()
    }
}
