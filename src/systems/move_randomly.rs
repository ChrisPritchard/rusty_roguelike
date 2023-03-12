use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovesRandomly)]
#[read_component(Player)]
#[read_component(Health)]
pub fn move_randomly(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer) {

    let (player, player_pos, _) = <(Entity, &Point, &Player)>::query().iter(ecs).nth(0).unwrap();

    let mut rng = RandomNumberGenerator::new();
    <(Entity, &Point, &MovesRandomly)>::query()
        .iter(ecs)
        .for_each(|(e, p, _)| {
            let delta = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };
            let np = *p + delta;

            if np == *player_pos {
                commands.push(((), WantsToAttack { attacker: *e, victim: *player})); 
            } else {
                commands.push(((), WantsToMove { entity: *e, destination: np})); 
            }            
        });
}