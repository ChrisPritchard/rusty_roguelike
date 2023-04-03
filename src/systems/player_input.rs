use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Weapon)]
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
        n if n >= VirtualKeyCode::Key1 || n <= VirtualKeyCode::Key9 => {
            use_item(n as usize, ecs, commands);
            Point::zero()
        }
        _ => Point::zero(),
    };

    if delta == Point::zero() {
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

    <(Entity, &Item, &Point)>::query().iter(ecs).filter(|(_, _, &pos)| pos == player_pos).for_each(|(entity, _, _)| {
        commands.remove_component::<Point>(*entity);
        commands.add_component(*entity, Carried(player));

        let entry_ref = ecs.entry_ref(*entity).unwrap();
        if entry_ref.get_component::<Weapon>().is_ok() {
            <(Entity, &Carried, &Weapon)>::query().iter(ecs).filter(|(_, c, _)| c.0 == player).for_each(|(weapon_entity, _, _)| {
                commands.remove(*weapon_entity);
            });
        }
    });
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let player = <(Entity, &Player)>::query().iter(ecs).find_map(|(e, _)| Some(*e)).unwrap();
    let item = <(Entity, &Item, &Carried)>::query().iter(ecs)
        .filter(|(_, _, c)| c.0 == player)
        .enumerate()
        .filter(|(item_count, _)| *item_count == n)
        .find_map(|(_, (e, _, _))| Some(*e));

    if let Some(item) = item {
        let entry_ref = ecs.entry_ref(item).unwrap();
        if entry_ref.get_component::<Weapon>().is_ok() {
            return; // can't use up weapons
        }
        commands.push(((), ActivateItem {
            used_by: player,
            item
        }));
    }
}
