use crate::{
    elements::Elem,
    generic::json::{FromJson, ToJson},
};

pub mod elements;
pub mod generic;

fn main() {
    let xs = Elem::note("Hello, World!").to_json();
    match Elem::from_json(xs.to_string()) {
        Ok(v) => println!("{:?} = {:?}", xs, v),
        Err(_) => todo!(),
    }
}
