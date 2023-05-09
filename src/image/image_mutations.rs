use uuid::Uuid;

use crate::element::element::Element;

pub trait ImageAdds
where
    Self: Sized,
{
    fn add_element(&self, _el: Element) -> Uuid {
        todo!()
    }
}

pub trait ImageDeletes
where
    Self: Sized,
{
    fn delete_element(&self, _id: Uuid) {
        todo!()
    }
}

pub trait ImageMutations: ImageAdds + ImageDeletes {}
