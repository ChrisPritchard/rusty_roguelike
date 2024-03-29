use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] map_theme: &Box<dyn  MapTheme>) {

    let fov = <&FieldOfView>::query().filter(component::<Player>()).iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let p = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if !map.in_bounds(p) {
                continue;
            }
            
            let glyph = map_theme.tile_to_render(map.tiles[map_idx(x, y)]);

            if fov.visible_tiles.contains(&p) {
                draw_batch.set(p - offset, ColorPair::new(WHITE, BLACK), glyph);
            } else if map.revealed[map_idx(x, y)] {
                draw_batch.set(p - offset, ColorPair::new((100, 100, 100), BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}