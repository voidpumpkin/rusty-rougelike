use crate::{
    components::{
        BlocksTile, CombatStats, InBackpack, Item, Monster, Name, Player, Position, Potion,
        Renderable, SufferDamage, Viewshed, WantsToDrinkPotion, WantsToDropItem, WantsToMelee,
        WantsToPickupItem,
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
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToDrinkPotion>();
    gs.ecs.register::<WantsToDropItem>();
}
