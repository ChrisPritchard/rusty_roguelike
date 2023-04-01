
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TurnState {
    AwaitingInput, PlayerTurn, EnemyTurn, GameOver, Victory, NextLevel
}