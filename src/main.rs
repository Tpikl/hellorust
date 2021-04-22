use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

mod components;
pub use components::*;

mod player;
pub use player::*;

mod map;
pub use map::*;



#[derive(Component)]
struct LeftMover {}

// Game state.
pub struct State {
    ecs: World
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        // Clear terminal.
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        // Draw things
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        // Render things.
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}


struct LeftWalker {}
impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty,pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0
                { pos.x = 79; }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}


// Game.
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // Create state with new world.
    let mut gs = State{
        ecs: World::new()
    };
    // Register components with the world.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<LeftMover>();

    // Insert the world.
    gs.ecs.insert(new_map());

    // Create player entity.
    gs.ecs.create_entity()
        .with(Position {
            x: 40,
            y: 25
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK)
        })
        .with(Player {})
        .build();

    // Create smilies. -- No smilies right now.
    for i in 0..0 {
        gs.ecs.create_entity()
            .with(Position {
                x: i * 7,
                y: 20
            })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK)
            })
            .with(LeftMover{})
            .build();
        }

    // Run game loop.
    rltk::main_loop(context, gs)
}
