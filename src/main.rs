mod components;
mod map;
mod player;
mod rect;
mod register_components;
mod spawn;
mod state;
mod systems;

use map::{draw_map, Map};
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
    let mut gs = State { ecs: World::new() };
    register_components(&mut gs);
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    let (player_entity,) = spawn(&mut gs, &map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(map);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    rltk::main_loop(context, gs)
}
