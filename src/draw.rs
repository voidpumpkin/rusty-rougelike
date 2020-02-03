use crate::{
    components::{Player, Position, Renderable},
    State,
};
use rltk::RGB;
use specs::prelude::{Builder, WorldExt};

fn draw_player(gs: &mut State) {
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();
}
fn _draw_enemies(gs: &mut State) {
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }
}
pub fn draw(gs: &mut State) {
    draw_player(gs);
    // draw_enemies(gs);
}