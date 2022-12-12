pub mod game;
use game::{Direction, State};

pub mod precompute;
use precompute::{
    get_possible_moves, load_precomputed, precompute, Precomputed, TranspositionTable,
};

pub mod expectimax;
#[allow(unused_imports)]
use expectimax::get_expectimax_move;

pub mod random;
#[allow(unused_imports)]
use random::get_random_move;

#[allow(dead_code)]
fn run_random_game(precomputed: &Precomputed) -> (u64, u64) {
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

#[allow(dead_code)]
fn run_expectimax_game(
    precomputed: &Precomputed,
    transposition: &mut TranspositionTable,
) -> (u64, u64) {
    let mut num_moves = 0;
    let mut state = State::new();
    let mut moves = get_possible_moves(state, precomputed);
    while moves[0].0 != Direction::Invalid {
        let score = state.get_score();
        let mut depth = 2;
        if score > 5000 {
            depth = 3;
        }
        if score > 10000 {
            depth = 4;
        }
        if score > 50000 {
            depth = 5;
        }
        if score > 100000 {
            depth = 6;
        }
        let (_move, new_state) =
            get_expectimax_move(state, moves, depth, precomputed, transposition);
        state = new_state.add_random_tile();
        num_moves += 1;
        moves = get_possible_moves(state, precomputed);
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
    let transposition: &mut TranspositionTable = &mut TranspositionTable::new();
    println!("Loaded precomputed data!");

    let start = std::time::Instant::now();
    let time = 10;

    let mut score = 0;
    let mut games = 0;
    let mut moves = 0;
    while start.elapsed().as_secs() < time {
        // let (temp_score, temp_moves) = run_random_game(precomputed);
        let (temp_score, temp_moves) = run_expectimax_game(precomputed, transposition);
        score += temp_score;
        moves += temp_moves;
        games += 1;
    }

    let time = start.elapsed().as_secs();

    println!("Games / Sec: {}", games as f64 / time as f64);
    println!("Moves / Sec: {}", moves as f64 / time as f64);
    println!("Score / Game: {}", score as f64 / games as f64);
}
