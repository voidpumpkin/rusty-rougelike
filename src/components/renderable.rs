use rltk::{FontCharType, RGB};
use specs::prelude::{Component, DenseVecStorage};

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
