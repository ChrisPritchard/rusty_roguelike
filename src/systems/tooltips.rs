use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
) {
    let mut positions = <(Entity, &Point, &Name)>::query(); // every entity that has a name and a position (not necessarily just enemies)
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let visible = <(&Player, &FieldOfView)>::query().iter(ecs).map(|(_, f)| &f.visible_tiles).nth(0).unwrap();
    if !visible.contains(&map_pos) {
        return;
    }

    positions
        .iter(ecs).filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, _, name)| {

            let screen_pos = *mouse_pos * 4;
            let display = 
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current) // if they have health as well (e.g. enemies) print that
                } else {
                    name.0.clone() // else just the name (e.g. items)
                };
            
            draw_batch.print(screen_pos, &display);
        });

    draw_batch.submit(10100).expect("Batch error");
}