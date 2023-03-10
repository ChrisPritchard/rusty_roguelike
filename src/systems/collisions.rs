use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let player_pos = <&Point>::query().filter(component::<Player>()).iter(ecs).nth(0).unwrap();
    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, p)| *p == player_pos)
        .for_each(|(e, _)| commands.remove(*e));
}