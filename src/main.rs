mod components;
mod gamelog;
mod gui;
mod map;
mod player;
mod rect;
mod register_components;
mod spawn;
mod state;
mod systems;

use map::Map;
use register_components::register_components;
use rltk::{Point, RltkBuilder, RltkError};
use spawn::spawn;
use specs::{World, WorldExt};
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
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()],
    });
    rltk::main_loop(context, gs)
}
