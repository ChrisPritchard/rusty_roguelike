pub use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let player_health = <(&Player, &Health)>::query().iter(ecs).map(|(_, h)| h.current).nth(0).unwrap();
    let new_state = match turn_state {
        _ if player_health <= 0 => TurnState::GameOver,
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        _ => *turn_state
    };
    *turn_state = new_state;
}