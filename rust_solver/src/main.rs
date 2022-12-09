// correctly import into main.rs
mod game;
use game::{get_initial_state, is_game_over, move_state, print_board};

mod algorithms {
    pub mod random;
}
use algorithms::random::get_random_move;

fn main() {
    let mut state = get_initial_state();
    print_board(state);

    while !is_game_over(state) {
        let move_ = get_random_move(state);
        state = move_state(state, move_, true);
        print_board(state);
    }
}
