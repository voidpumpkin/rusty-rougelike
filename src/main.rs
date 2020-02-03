#[macro_use]
extern crate specs_derive;

mod components;
mod draw;
mod map;
mod player;
mod state;
mod systems;

pub use draw::draw;
pub use map::{draw_map, new_map, xy_idx, TileType};
pub use player::player_input;
pub use state::State;

use components::{Player, Position, Renderable};
use rltk::RltkBuilder;
use specs::prelude::{World, WorldExt};

fn main() {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();
    let mut gs = State { ecs: World::new() };
    register_components(&mut gs);
    gs.ecs.insert(new_map());
    draw(&mut gs);
    rltk::main_loop(context, gs);
}

fn register_components(gs: &mut State) {
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
}
