mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use std::collections::HashMap;

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: HashMap<TurnState, Schedule>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0); // background / dungeon tiles
        ctx.cls();
        ctx.set_active_console(1); // player, monsters, items
        ctx.cls();
        ctx.set_active_console(2); // ui / hud
        ctx.cls();

        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        if current_state == TurnState::GameOver {
            self.game_over(ctx)
        } else {
            self.systems.entry(current_state).and_modify(|schedule| 
                schedule.execute(&mut self.ecs, &mut self.resources));
        }
        render_draw_buffer(ctx).expect("Render error");

        if let Some(VirtualKeyCode::Escape) = ctx.key {
            ctx.quitting = true;
        }
    }
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        map_builder.rooms.iter().skip(1).for_each(|r| {
            let pos = r.center();
            match rng.roll_dice(1, 10) {
                1..=6 => spawn_goblin(&mut ecs, pos),
                7..=8 => spawn_drunk_goblin(&mut ecs, pos),
                _ => spawn_orc(&mut ecs, pos)
            };
        });
        
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);

        let systems = HashMap::from([
           (TurnState::AwaitingInput, build_input_scheduler()), 
           (TurnState::PlayerTurn, build_player_scheduler()),
           (TurnState::EnemyTurn, build_enemy_scheduler()),
        ]);

        Self { ecs, resources, systems }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended");
        ctx.print_color_centered(4, WHITE, BLACK, "Slain by a monster, your hero's journey has come to a premature end.");
        ctx.print_color_centered(5, WHITE, BLACK, "The Amulet of Yala remains unclaimed, and your home town is not saved.");

        ctx.print_color_centered(8, YELLOW, BLACK, "Don't worry, you can always try again with a new hero.");
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            let new_game = State::new();
            self.ecs = new_game.ecs;
            self.resources = new_game.resources;
            self.systems = new_game.systems;
        }
    }
}

fn main() -> BError {
    let context = 
        BTermBuilder::new()
        .with_title("Dungeoncrawl")
        .with_fps_cap(30.)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH*2, SCREEN_HEIGHT*2, "terminal8x8.png") // four times as many characters as normal with this font
        .build()?;
    main_loop(context, State::new())
}
