use crate::game::Direction;
use crate::precompute::{get_possible_moves, Precomputed};

pub fn get_random_move(state: u64, precomputed: &Precomputed) -> Direction {
    let mut moves = get_possible_moves(state, precomputed);
    let index = rand::random::<usize>() % moves.len();
    moves.remove(index)
}
