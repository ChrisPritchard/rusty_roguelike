use crate::prelude::*;

mod player_input;
mod map_render;
mod entity_render;
mod collisions;
mod move_randomly;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(move_randomly::move_randomly_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}