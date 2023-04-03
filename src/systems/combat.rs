use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let victims: Vec<(Entity, Entity, Entity)> = 
        <(Entity, &WantsToAttack)>::query().iter(ecs)
            .map(|(message, attack_order)| (*message, attack_order.attacker, attack_order.victim))
            .collect();
    for (message, attacker, victim) in victims {
        let mut damage = 0; 
        {
            let attacker_ref = ecs.entry_ref(attacker).unwrap();
            if let Ok(dmg) = attacker_ref.get_component::<Damage>() { 
                damage += dmg.0
            }

            // add weapon damage
            <(&Carried, &Damage)>::query().iter(ecs).filter(|(carried, _)| carried.0 == attacker).for_each(|(_, d)| damage += d.0);
        }
        let mut victim_entity = ecs.entry_mut(victim).unwrap();        

        if let Ok(mut health) = victim_entity.get_component_mut::<Health>() {
            health.current -= damage;
            if health.current < 1 && victim_entity.get_component::<Player>().is_err() {
                commands.remove(victim);
            }
        }
        commands.remove(message);
    }
}