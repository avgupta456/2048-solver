pub mod game;
use game::{add_random_tile, get_initial_state, get_score};

pub mod precompute;
use precompute::{get_possible_moves, load_precomputed, precompute, Precomputed};

pub mod algorithms {
    pub mod random;
}
use algorithms::random::get_random_move;

fn run_game(precomputed: &Precomputed) -> (u64, u64) {
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
