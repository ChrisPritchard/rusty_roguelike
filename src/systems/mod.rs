use crate::prelude::*;

mod player_input;
mod map_render;
mod entity_render;
mod collisions;
mod move_randomly;
mod end_turn;
mod movement;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_enemy_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(move_randomly::move_randomly_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        // .add_system(collisions::collisions_system())
        // .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}