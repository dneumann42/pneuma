#[cfg(test)]
mod queries {
    use crate::{
        element::{Element, Heading, Note},
        generic::Kind,
        image::{Image, Mode},
        sqlite_image::SqliteImage,
    };

    #[test]
    fn it_can_query_kind() {
        let mut img = SqliteImage::new("test".to_owned());
        img.load(Mode::Memory);
        let a = img.add_element(Element::note("".to_string(), "".to_string()));
        let b = img.add_element(Element::heading(1));
        assert_eq!(Note::kind_const(), img.get_element_kind_by_id(a));
        assert_eq!(Heading::kind_const(), img.get_element_kind_by_id(b));
    }

    fn it_can_query_note() {}

    fn it_can_query_by_id() {}

    #[test]
    fn it_can_query_all_notes() {
        let mut img = SqliteImage::new("test".to_owned());
        img.load(Mode::Memory);

        let xs = vec![
            Element::note("Hwllo".to_string(), "Des".to_string()),
            Element::note("Hwllo1".to_string(), "Des".to_string()),
            Element::note("Hwllo2".to_string(), "Des".to_string()),
            Element::note("Hwllo3".to_string(), "Des".to_string()),
        ];

        img.add_element(xs[0].clone());
        img.add_element(xs[1].clone());
        img.add_element(xs[2].clone());
        img.add_element(xs[3].clone());

        let ns = img.get_all_notes();
        assert_eq!(ns.len(), xs.len());
    }
}
