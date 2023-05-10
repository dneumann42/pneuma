use pneuma::{
    elements::Elem,
    generic::json::{FromJson, ToJson},
};

#[test]
fn it_can_serialize_deserialize_notes() {
    let xs = Elem::note("Hello, World!").to_json();
    match Elem::from_json(xs.to_string()) {
        Ok(v) => {
            assert_eq!(v.to_json(), xs)
        }
        Err(_) => todo!(),
    }
}
