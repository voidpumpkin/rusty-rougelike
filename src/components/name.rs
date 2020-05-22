use specs::{Component, DenseVecStorage};
use specs_derive::Component;
#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}
