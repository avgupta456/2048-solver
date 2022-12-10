use crate::game::Direction;
use crate::precompute::Precomputed;

pub fn get_random_move(
    _state: u64,
    moves: Vec<(Direction, u64)>,
    _precomputed: &Precomputed,
) -> (Direction, u64) {
    let index = rand::random::<usize>() % moves.len();
    moves[index]
}
