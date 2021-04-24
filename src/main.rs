use rltk::{Rltk, GameState, RGB};
use specs::prelude::*;
use specs_derive::Component;

mod components;
pub use components::*;
mod player;
pub use player::*;
mod map;
pub use map::*;
mod rect;
pub use rect::Rect;


#[derive(Component)]
struct LeftMover {}

// Game state.
pub struct State {
    ecs: World
}
impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
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
    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    // Create player entity.
    gs.ecs.create_entity()
        .with(Position {x: player_x, y: player_y})
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
