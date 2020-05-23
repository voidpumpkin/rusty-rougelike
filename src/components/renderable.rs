use rltk::{FontCharType, RGB};
use specs::{Component, DenseVecStorage};

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}
