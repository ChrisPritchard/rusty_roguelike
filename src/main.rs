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

use std::collections::{HashMap, HashSet};

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
        match current_state {
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
            _ => {
                self.systems.entry(current_state).and_modify(|schedule| schedule.execute(&mut self.ecs, &mut self.resources));
            },
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
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        //spawn_amulet_of_yara(&mut ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);
        
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        let systems = HashMap::from([
           (TurnState::AwaitingInput, build_input_scheduler()), 
           (TurnState::PlayerTurn, build_player_scheduler()),
           (TurnState::EnemyTurn, build_enemy_scheduler()),
        ]);

        Self { ecs, resources, systems }
    }

    fn reset_game_state(&mut self) {
        let new_game = State::new();
        self.ecs = new_game.ecs;
        self.resources = new_game.resources;
        self.systems = new_game.systems;
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended");
        ctx.print_color_centered(4, WHITE, BLACK, "Slain by a monster, your hero's journey has come to a premature end.");
        ctx.print_color_centered(5, WHITE, BLACK, "The Amulet of Yala remains unclaimed, and your home town is not saved.");

        ctx.print_color_centered(8, YELLOW, BLACK, "Don't worry, you can always try again with a new hero.");
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(4, WHITE, BLACK, "You put on the Amulet of Yala and feel its power course through your veins.");

        ctx.print_color_centered(5, YELLOW, BLACK, "Your town is saved, and you can return to your normal life");
        ctx.print_color_centered(6, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query().filter(component::<Player>()).iter(&mut self.ecs).nth(0).unwrap();
        
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query().iter(&self.ecs).filter(|(_, carried)| carried.0 == player_entity).map(|(e, _)| *e).for_each(|e| { entities_to_keep.insert(e); });

        let mut commands = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                commands.remove(*e);
            }
        }
        commands.flush(&mut self.ecs, &mut self.resources);

        <&mut FieldOfView>::query().iter_mut(&mut self.ecs).for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query().iter_mut(&mut self.ecs).for_each(|(player, point)| {
            player.map_level += 1;
            map_level = player.map_level;
            point.x = map_builder.player_start.x;
            point.y = map_builder.player_start.y;
        });

        if map_level == 2 {
            spawn_amulet_of_yara(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_level(&mut self.ecs, &mut rng, map_level as usize, &map_builder.monster_spawns);
        
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
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
