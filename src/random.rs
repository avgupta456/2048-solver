use super::game::{Direction, State};
use super::precompute::Precomputed;

#[allow(dead_code)]
pub fn get_random_move(
    _state: State,
    moves: [(Direction, State); 4],
    _precomputed: &Precomputed,
) -> (Direction, State) {
    let index = rand::random::<usize>() % moves.len();
    moves[index]
}
