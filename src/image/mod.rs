use uuid::Uuid;

use crate::element::Element;

pub trait Image
where
    Self: Sized,
{
    fn load(&mut self) {}
    fn close(self) {}

    fn query_element_by_id(&self, _: Uuid) -> Option<Element> {
        None
    }

    fn add_element(&self, _: Element) {}
}
