pub mod game;
use game::State;

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

#[allow(dead_code)]
fn run_random_game(precomputed: &Precomputed) -> (u64, u64) {
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

#[allow(dead_code)]
fn run_expectimax_game(precomputed: &Precomputed) -> (u64, u64) {
    let mut num_moves = 0;
    let mut state = State::new();
    let mut moves = get_possible_moves(state, precomputed);
    while moves.len() > 0 {
        let (_move, new_state) = get_expectimax_move(state, moves, 4, precomputed);
        state = new_state.add_random_tile();
        moves = get_possible_moves(state, precomputed);
        num_moves += 1;
        state.print_board()
    }
    println!("Score: {}", state.get_score());
    (state.get_score(), num_moves)
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
        let (temp_score, temp_moves) = run_random_game(precomputed);
        score += temp_score;
        moves += temp_moves;
        games += 1;
    }

    println!("Games / Sec: {}", games as f64 / time as f64);
    println!("Moves / Sec: {}", moves as f64 / time as f64);
    println!("Score / Game: {}", score as f64 / games as f64);
}
