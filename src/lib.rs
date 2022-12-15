/*
Purely for benchmarking, code duplicated from main.rs
Need to manually update this if main.rs changes
*/

pub mod game;
use game::{Direction, State};

pub mod precompute;
use precompute::{get_possible_moves, load_precomputed as _load_precomputed, Precomputed};

pub mod expectimax;
#[allow(unused_imports)]
use expectimax::get_expectimax_move;

pub mod random;
#[allow(unused_imports)]
use random::get_random_move;

pub fn load_precomputed() -> Precomputed {
    _load_precomputed()
}

pub fn run_game(precomputed: &Precomputed) -> (u64, u64) {
    let mut num_moves = 0;
    let mut state = State::new();
    let mut moves = get_possible_moves(state, precomputed);
    while moves[0].0 != Direction::Invalid {
        let (_move, new_state) = get_random_move(state, moves, precomputed);
        state = new_state.add_random_tile();
        num_moves += 1;
        moves = get_possible_moves(state, precomputed);
    }
    (state.get_score(), num_moves)
}
