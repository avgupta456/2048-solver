mod game;
use game::{get_initial_state, get_score};

mod precompute;
use precompute::{is_game_over, load_precomputed, move_state, precompute, Precomputed};

mod algorithms {
    pub mod random;
}
use algorithms::random::get_random_move;

fn run_game(precomputed: &Precomputed) -> (u64, u64) {
    let mut num_moves = 0;
    let mut state = get_initial_state();
    while !is_game_over(state, precomputed) {
        let move_ = get_random_move(state, precomputed);
        state = move_state(state, move_, true, precomputed);
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
