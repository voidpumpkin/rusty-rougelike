use rltk::Point;
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}
