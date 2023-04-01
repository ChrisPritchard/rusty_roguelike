pub use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(AmuletOfYara)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {

    let (player_health, player_position) = <(&Player, &Health, &Point)>::query()
        .iter(ecs).map(|(_, h, p)| (h.current, p)).nth(0).unwrap();
    let amulet_position = <(&AmuletOfYara, &Point)>::query()
        .iter(ecs).map(|(_, p)| p).nth(0);
    let player_tile = map.tiles[map.point2d_to_index(*player_position)];

    let new_state = match turn_state {
        _ if player_health <= 0 => TurnState::GameOver,
        _ if amulet_position.is_some() && player_position == amulet_position.unwrap() => TurnState::Victory,
        _ if player_tile == TileType::Exit => TurnState::NextLevel,
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        _ => *turn_state
    };
    
    *turn_state = new_state;
}