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
    depth: u16,
    precomputed: &Precomputed,
    transposition: &mut TranspositionTable,
) -> (u64, u64) {
    let mut num_moves = 0;
    let mut state = State::new();
    let mut moves = get_possible_moves(state, precomputed);
    while moves[0].0 != Direction::Invalid {
        let (_move, new_state) =
            get_expectimax_move(state, moves, depth, precomputed, transposition);
        state = new_state.add_random_tile();
        num_moves += 1;
        moves = get_possible_moves(state, precomputed);
        state.print_board()
    }
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

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: rust_solver.exe <depth>");
        return;
    }
    let depth = args[1].parse::<u16>().unwrap();

    let start = std::time::Instant::now();
    let (score, moves) = run_expectimax_game(depth, precomputed, transposition);
    let time = start.elapsed().as_millis() as f32 / 1000.0;

    println!("---");
    println!("Score:   \t{}", score);
    println!("Moves:   \t{}", moves);
    println!("Time:    \t{}s", (time * 1000.0).round() / 1000.0);
    println!("Moves/s: \t{}", (moves as f32 / time).round());
}
