use crate::prelude::*;

// wantstomove is a message, passed about as an entity; the entity parameter below only contains a single component of type wantstomove
// therefore the entity value below is only used to remove the message. instead wantstomove.entity is used to establish which real entity 
// is being affected.

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    if map.can_enter_tile(want_move.destination) {
        // this replaces the point that represents the entities current position, with a the destination point
        // taking advantage of the components on an entity basically being a type set
        commands.add_component(want_move.entity, want_move.destination); 
        
        let entry = ecs.entry_ref(want_move.entity).unwrap();
        let fov = entry.get_component::<FieldOfView>();

        if entry.get_component::<Player>().is_ok() {
            camera.on_player_move(want_move.destination);
            for p in fov.unwrap().visible_tiles.iter() {
                let idx = map.point2d_to_index(*p);
                map.revealed[idx] = true;
            }
        }
        
        if let Ok(fov) = fov {
            commands.add_component(want_move.entity, fov.clone_dirty()); 
        }
    }
    commands.remove(*entity);
}