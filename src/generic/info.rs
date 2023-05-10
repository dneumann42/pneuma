pub trait Title {
    fn title(&self) -> &String;
}

pub trait Descr {
    fn descr(&self) -> &String;
}

pub trait TitleDescr: Title + Descr {}

pub trait UniqueId {
    fn uid(&self) -> String;
}