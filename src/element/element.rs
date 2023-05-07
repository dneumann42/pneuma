use serde_json::{json, Value};
use uuid::Uuid;

use crate::generic::{Kind, ToJson, UniqueId};

use super::{fragment::Fragment, heading::Heading, note::Note};

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

    pub fn note(title: &String, descr: &String) -> Self {
        Self::new(Fragment::Note(Note::new(title, descr)))
    }

    pub fn heading(heading_level: i32) -> Self {
        Self::new(Fragment::Heading(Heading::HeadingLevel(heading_level)))
    }
}

impl ToJson for Element {
    fn to_json(&self) -> Value {
        json!({
            "kind": "element",
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
