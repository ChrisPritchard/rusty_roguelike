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
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.resources.insert(ctx.key);
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        self.systems.entry(current_state).and_modify(|schedule| 
            schedule.execute(&mut self.ecs, &mut self.resources));
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
        map_builder.rooms.iter().skip(1).for_each(|r| spawn_monster(&mut ecs, r.center(), rng.range(0, 4)) );
        
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
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;
    main_loop(context, State::new())
}
