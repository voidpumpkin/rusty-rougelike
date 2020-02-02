use crate::components::{LeftMover, Player, Position, Renderable};
use crate::state::State;
use rltk::RGB;
use specs::prelude::{Builder, WorldExt};

fn spawn_player(gs: &mut State) {
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
fn sapwn_enemies(gs: &mut State) {
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }
}
pub fn run_spawners(gs: &mut State) {
    spawn_player(gs);
    sapwn_enemies(gs);
}
