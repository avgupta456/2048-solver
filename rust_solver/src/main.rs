pub mod game;
use game::{add_random_tile, get_initial_state, get_score};

pub mod precompute;
use precompute::{get_possible_moves, load_precomputed, precompute, Precomputed};

pub mod algorithms {
    pub mod expectimax;
    pub mod random;
}
#[allow(unused_imports)]
use algorithms::expectimax::get_expectimax_move;
#[allow(unused_imports)]
use algorithms::random::get_random_move;
use rust_solver::game::print_board;

fn run_game(precomputed: &Precomputed) -> (u64, u64) {
    let mut num_moves = 0;
    let mut state = get_initial_state();
    let mut moves = get_possible_moves(state, precomputed);
    while moves.len() > 0 {
        let curr_score = get_score(state);
        let mut depth = 2;
        if curr_score >= 10000 {
            depth = 3;
        }
        if curr_score >= 50000 {
            depth = 4;
        }
        let (_move, new_state) = get_expectimax_move(state, moves, depth, precomputed);
        state = add_random_tile(new_state);
        moves = get_possible_moves(state, precomputed);
        num_moves += 1;
        print_board(state)
    }
    println!("Score: {}", get_score(state));
    (get_score(state), num_moves)
}

fn main() {
    if !std::path::Path::new("precomputed.json").exists() {
        println!("Precomputing...");
        precompute();
    }
    let precomputed: &Precomputed = &load_precomputed();
    println!("Loaded precomputed data!");

    let start = std::time::Instant::now();
    let time = 10;

    let mut score = 0;
    let mut games = 0;
    let mut moves = 0;
    while start.elapsed().as_secs() < time {
        let (temp_score, temp_moves) = run_game(precomputed);
        score += temp_score;
        moves += temp_moves;
        games += 1;
    }

    println!("Games / Sec: {}", games as f64 / time as f64);
    println!("Moves / Sec: {}", moves as f64 / time as f64);
    println!("Score / Game: {}", score as f64 / games as f64);
}
