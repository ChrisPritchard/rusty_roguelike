use crate::prelude::*;

// wantstomove is a message, passed about as an entity; the entity parameter below only contains a single component of type wantstomove
// therefore the entity value below is only used to remove the message. instead wantstomove.entity is used to establish which real entity 
// is being affected.

#[system(for_each)]
#[read_component(Player)]
#[read_component(Point)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    let occupied_count = <&Point>::query().iter(ecs).filter(|p| **p == want_move.destination).count();
    if map.can_enter_tile(want_move.destination) && occupied_count == 0 {
        commands.add_component(want_move.entity, want_move.destination);

        if ecs.entry_ref(want_move.entity).unwrap().get_component::<Player>().is_ok() {
            camera.on_player_move(want_move.destination);
        }
    }
    commands.remove(*entity);
}