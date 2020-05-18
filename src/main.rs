#[macro_use]
extern crate specs_derive;

mod components;
mod map;
mod player;
mod rect;
mod register_components;
mod spawn;
mod state;
mod systems;

use map::{draw_map, Map, TileType};
use player::player_input;
use rect::Rect;
use register_components::register_components;
use rltk::{Point, RltkBuilder, RltkError};
use spawn::spawn;
use specs::prelude::{World, WorldExt};
use state::{RunState, State};

fn main() -> RltkError {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };
    register_components(&mut gs);
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(Point::new(player_x, player_y));
    spawn(&mut gs, &map);
    gs.ecs.insert(map);
    rltk::main_loop(context, gs)
}
