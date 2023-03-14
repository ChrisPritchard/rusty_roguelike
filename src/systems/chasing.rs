use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
pub fn chasing(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    let (player, _,  player_pos) = <(Entity, &Player, &Point)>::query().iter(ecs).nth(0).unwrap();
    let search_targets = vec![map_idx(player_pos.x, player_pos.y)];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    <(Entity, &Point, &ChasingPlayer)>::query().iter(ecs).for_each(|(entity, pos, _)| {
        let start = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, start, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else { *player_pos }; // next to player

            if destination == *player_pos {
                commands.push(((), WantsToAttack { attacker: *entity, victim: *player})); 
            } else {
                commands.push(((), WantsToMove { entity: *entity, destination})); 
            }   
        }
    });
}