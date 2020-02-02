use rltk::RGB;
use specs::prelude::{Component, DenseVecStorage};

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}
