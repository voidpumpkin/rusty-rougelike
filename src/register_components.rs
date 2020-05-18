use crate::{
    components::{Monster, Name, Player, Position, Renderable, Viewshed},
    state::State,
};
use specs::prelude::WorldExt;

pub fn register_components(gs: &mut State) {
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
}
