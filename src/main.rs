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
pub use map::{draw_map, Map, TileType};
pub use player::player_input;
pub use rect::Rect;
pub use state::State;

use components::{Player, Position, Renderable, Viewshed};
use rltk::{RltkBuilder, RltkError};
use specs::prelude::{World, WorldExt};
use systems::VisibilitySystem;

fn main() -> RltkError {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    register_components(&mut gs);
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);
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
    gs.ecs.register::<Viewshed>();
}
