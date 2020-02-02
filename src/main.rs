#[macro_use]
extern crate specs_derive;

mod components;
mod spawners;
mod state;
mod systems;
mod utils;

use components::{LeftMover, Player, Position, Renderable};
use rltk::RltkBuilder;
use spawners::run_spawners;
use specs::prelude::{World, WorldExt};
use state::State;

fn main() {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();
    let mut gs = State { ecs: World::new() };
    register_components(&mut gs);
    run_spawners(&mut gs);
    rltk::main_loop(context, gs);
}

fn register_components(gs: &mut State) {
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
}
