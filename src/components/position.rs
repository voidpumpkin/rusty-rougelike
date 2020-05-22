use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
