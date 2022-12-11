use crate::game::{Direction, State};
use crate::precompute::Precomputed;

#[allow(dead_code)]
pub fn get_random_move(
    _state: State,
    moves: Vec<(Direction, State)>,
    _precomputed: &Precomputed,
) -> (Direction, State) {
    let index = rand::random::<usize>() % moves.len();
    moves[index]
}
