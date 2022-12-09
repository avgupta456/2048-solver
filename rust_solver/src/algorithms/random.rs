use crate::game::{get_possible_moves, Direction};

pub fn get_random_move(state: u64) -> Direction {
    let mut moves = get_possible_moves(state);
    let index = rand::random::<usize>() % moves.len();
    moves.remove(index)
}
