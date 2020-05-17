#[macro_use]
extern crate specs_derive;

mod components;
mod draw;
mod map;
mod player;
mod rect;
mod state;
mod systems;

pub use draw::draw;
pub use map::{draw_map, new_map_rooms_and_corridors, new_map_test, xy_idx, TileType};
pub use player::player_input;
pub use rect::Rect;
pub use state::State;

use components::{Player, Position, Renderable};
use rltk::{RltkBuilder, RltkError};
use specs::prelude::{World, WorldExt};

fn main() -> RltkError {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    register_components(&mut gs);
    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();
    draw(
        &mut gs,
        Position {
            x: player_x,
            y: player_y,
        },
    );
    rltk::main_loop(context, gs)
}

fn register_components(gs: &mut State) {
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
}
