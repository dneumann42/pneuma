use uuid::Uuid;

use crate::generic::{Descr, Kind, Title, TitleDescr, UniqueId};

#[derive(Clone, Debug)]
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

impl Kind for Heading {
    fn kind(&self) -> String {
        "heading".to_owned()
    }
}

#[derive(Clone, Debug)]
pub struct Note {
    pub title: String,
    pub descr: String,
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

impl Kind for Note {
    fn kind(&self) -> String {
        "note".to_owned()
    }
}

impl TitleDescr for Note {}

#[derive(Clone, Debug)]
pub enum Fragment {
    Note(Note),
    Heading(Heading),
}

impl Kind for Fragment {
    fn kind(&self) -> String {
        match self {
            Fragment::Note(_) => "note",
            Fragment::Heading(_) => "heading",
        }
        .to_owned()
    }
}

#[derive(Clone, Debug)]
pub struct Element {
    id: Uuid,
    fragment: Fragment,
}

impl Element {
    pub fn new(fragment: Fragment) -> Self {
        Element {
            id: Uuid::new_v4(),
            fragment,
        }
    }

    pub fn fragment(&self) -> &Fragment {
        &self.fragment
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
