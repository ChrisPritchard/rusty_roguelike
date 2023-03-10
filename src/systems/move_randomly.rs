use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovesRandomly)]
pub fn move_randomly(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut rng = RandomNumberGenerator::new();
    <(&mut Point, &MovesRandomly)>::query()
        .iter_mut(ecs)
        .for_each(|(p, _)| {
            let delta = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };
            let np = *p + delta;
            if map.can_enter_tile(np) {
                *p = np;
            }
        });
}