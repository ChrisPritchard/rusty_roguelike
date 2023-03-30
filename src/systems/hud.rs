use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let health = <&Health>::query().filter(component::<Player>()).iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    draw_batch.print_centered(1, "Explore the dungeon. Use cursor keys to move");
    draw_batch.bar_horizontal(Point::zero(), SCREEN_WIDTH*2, health.current, health.max, ColorPair::new(RED, BLACK));
    draw_batch.print_color_centered(0, format!( "Health: {} / {}", health.current, health.max), ColorPair::new(WHITE, RED));

    let player = <(Entity, &Player)>::query().iter(ecs).find_map(|(e, _)| Some(*e)).unwrap();
    let mut y = 3;
    <(&Item, &Name, &Carried)>::query().iter(ecs).filter(|(_, _, carried)| carried.0 == player).for_each(|(_, name, _)| {
        draw_batch.print(Point::new(3, y), format!("{} : {}", y-2, &name.0));
        y += 1;
    });

    if y > 3 {
        draw_batch.print_color(Point::new(3, 2), "Items carried", ColorPair::new(YELLOW, BLACK));
    }

    draw_batch.submit(10000).expect("batch error");
}