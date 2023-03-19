use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);

    let visible = <(&Player, &FieldOfView)>::query().iter(ecs).map(|(_, f)| &f.visible_tiles).nth(0).unwrap();

    <(&Point, &Render)>::query().iter(ecs).for_each(|(pos, render)| {
        if visible.contains(pos) {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        }
    });

    draw_batch.submit(5000).expect("Batch error");
}