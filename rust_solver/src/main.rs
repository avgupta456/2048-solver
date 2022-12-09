// correctly import into main.rs
mod game;
use core::num;

use game::{get_initial_state, get_score, is_game_over, move_state, print_board};

mod algorithms {
    pub mod random;
}
use algorithms::random::get_random_move;

fn run_game() -> (u64, u64) {
    let mut num_moves = 0;
    let mut state = get_initial_state();
    while !is_game_over(state) {
        let move_ = get_random_move(state);
        state = move_state(state, move_, true);
        num_moves += 1;
    }
    (get_score(state), num_moves)
}

fn main() {
    let start = std::time::Instant::now();
    let time = 10;

    let mut score = 0;
    let mut games = 0;
    let mut moves = 0;
    while start.elapsed().as_secs() < time {
        let (temp_score, temp_moves) = run_game();
        score += temp_score;
        moves += temp_moves;
        games += 1;
    }

    println!("Games / Sec: {}", games as f64 / time as f64);
    println!("Moves / Sec: {}", moves as f64 / time as f64);
    println!("Score / Game: {}", score as f64 / games as f64);
}
