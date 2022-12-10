/*
Purely for benchmarking, code duplicated from main.rs
Need to manually update this if main.rs changes
*/

pub mod game;
use game::{add_random_tile, get_initial_state, get_score};

pub mod precompute;
use precompute::{get_possible_moves, load_precomputed as _load_precomputed, Precomputed};

mod algorithms {
    pub mod random;
}
use algorithms::random::get_random_move;

pub fn load_precomputed() -> Precomputed {
    _load_precomputed()
}

pub fn run_game(precomputed: &Precomputed) -> (u64, u64) {
    let mut num_moves = 0;
    let mut state = get_initial_state();
    let mut moves = get_possible_moves(state, precomputed);
    while moves.len() > 0 {
        let (_move, new_state) = get_random_move(state, moves, precomputed);
        state = add_random_tile(new_state);
        moves = get_possible_moves(state, precomputed);
        num_moves += 1;
    }
    (get_score(state), num_moves)
}
