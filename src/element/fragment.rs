use serde_json::Value;

use crate::generic::{Kind, ToJson};

use super::{heading::Heading, note::Note};

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
