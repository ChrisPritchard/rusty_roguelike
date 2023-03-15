use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let victims: Vec<(Entity, Entity)> = 
        <(Entity, &WantsToAttack)>::query().iter(ecs)
            .map(|(message, attack_order)| (*message, attack_order.victim))
            .collect();
    for (message, victim) in victims {
        let mut victim_entity = ecs.entry_mut(victim).unwrap();
        if let Ok(mut health) = victim_entity.get_component_mut::<Health>() {
            health.current -= 1;
            if health.current < 1 && victim_entity.get_component::<Player>().is_err() {
                commands.remove(victim);
            }
        }
        commands.remove(message);
    }
}