use crate::{
    components::{Position, Renderable},
    draw_map, player_input, TileType,
};
use rltk::{GameState, Rltk};
use specs::prelude::{Join, World, WorldExt};

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();
        player_input(self, ctx);
        let map = self.ecs.fetch::<Vec<TileType>>();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        draw_map(&map, ctx);
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
