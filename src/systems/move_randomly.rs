use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovesRandomly)]
pub fn move_randomly(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer) {

    let mut rng = RandomNumberGenerator::new();
    <(Entity, &Point, &MovesRandomly)>::query()
        .iter_mut(ecs)
        .for_each(|(e, p, _)| {
            let delta = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };
            let np = *p + delta;
            commands.push(((), WantsToMove { entity: *e, destination: np}));
        });
}