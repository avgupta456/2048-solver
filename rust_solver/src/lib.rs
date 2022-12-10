/*
Purely for benchmarking, code duplicated from main.rs
Need to manually update this if main.rs changes
*/

mod game;
use game::{get_initial_state, get_score};

mod precompute;
use precompute::{is_game_over, load_precomputed as _load_precomputed, move_state, Precomputed};

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
    while !is_game_over(state, precomputed) {
        let move_ = get_random_move(state, precomputed);
        state = move_state(state, move_, true, precomputed);
        num_moves += 1;
    }
    (get_score(state), num_moves)
}
