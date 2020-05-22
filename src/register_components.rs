use crate::{
    components::{
        BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, SufferDamage,
        Viewshed, WantsToMelee,
    },
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
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
}
