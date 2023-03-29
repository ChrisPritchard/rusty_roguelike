use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Item)]
#[write_component(Carried)]
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
        VirtualKeyCode::G => {
            try_grab_item(ecs, commands);
            Point::zero()
        }
        _ => Point::zero(),
    };

    if delta == Point::zero() {
        <(Entity, &mut Health)>::query().filter(component::<Player>()).iter_mut(ecs)
            .for_each(|(_, h)| (*h).current = ((*h).current + 1).min((*h).max));
        *turn_state = TurnState::PlayerTurn;
        return
    }

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

fn try_grab_item(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let (player, player_pos) = <(Entity, &Point)>::query().filter(component::<Player>()).iter(ecs).find_map(|(e, p)| Some((*e, *p))).unwrap();

    <(Entity, &Item, &Point)>::query().iter(ecs).filter(|(_, _, &pos)| pos == player_pos).for_each(|(e, _, _)| {
        commands.remove_component::<Point>(*e);
        commands.add_component(*e, Carried(player));
    });
}
