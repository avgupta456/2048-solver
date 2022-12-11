/*
Purely for benchmarking, code duplicated from main.rs
Need to manually update this if main.rs changes
*/

pub mod game;
use game::State;

pub mod precompute;
use precompute::{get_possible_moves, load_precomputed as _load_precomputed, Precomputed};

mod algorithms {
    pub mod expectimax;
    pub mod random;
}
#[allow(unused_imports)]
use algorithms::expectimax::get_expectimax_move;
#[allow(unused_imports)]
use algorithms::random::get_random_move;

pub fn load_precomputed() -> Precomputed {
    _load_precomputed()
}

pub fn run_game(precomputed: &Precomputed) -> (u64, u64) {
    let mut num_moves = 0;
    let mut state = State::new();
    let mut moves = get_possible_moves(state, precomputed);
    while moves.len() > 0 {
        let (_move, new_state) = get_random_move(state, moves, precomputed);
        state = new_state.add_random_tile();
        moves = get_possible_moves(state, precomputed);
        num_moves += 1;
    }
    (state.get_score(), num_moves)
}
