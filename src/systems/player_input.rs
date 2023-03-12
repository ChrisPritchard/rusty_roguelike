use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
        ecs: &mut SubWorld, 
        commands: &mut CommandBuffer,
        #[resource] key: &Option<VirtualKeyCode>,
        #[resource] turn_state: &mut TurnState) {

    if (*key).is_none() {
        return;
    }

    let delta = match (*key).unwrap() {
        VirtualKeyCode::Left => Point::new(-1, 0),
        VirtualKeyCode::Right => Point::new(1, 0),
        VirtualKeyCode::Up => Point::new(0, -1),
        VirtualKeyCode::Down => Point::new(0, 1),
        _ => return,
    };

    let (player, player_target) = <(Entity, &Point)>::query().filter(component::<Player>())
        .iter(ecs).find_map(|(e, p)| Some((e, *p + delta))).unwrap();

    let enemy = <(Entity, &Point)>::query().filter(component::<Enemy>())
        .iter(ecs).filter(|(_, p)| **p == player_target).find_map(|(e, _)| Some(e));

    if let Some(enemy) = enemy {
        commands.push(((), WantsToAttack{ attacker: *player, victim: *enemy }));
    } else {
        commands.push(((), WantsToMove{ entity: *player, destination: player_target }));
    }
    
    *turn_state = TurnState::PlayerTurn;
}