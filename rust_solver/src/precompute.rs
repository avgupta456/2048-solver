use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use crate::game::{
    add_random_tile, from_grid, get_cols, get_empty_tiles, get_rows, move_left, move_right,
    set_cols, set_rows, to_grid, transpose, Direction,
};

/*
PRECOMPUTATION INFRASTRUCTURE
*/

#[derive(Serialize, Deserialize)]
pub struct Precomputed {
    move_left: Vec<u16>,
    move_right: Vec<u16>,
}

fn _save_precomputed(file_name: String, data: Precomputed) {
    let json_string = to_string(&data).unwrap();
    std::fs::write(file_name, json_string).unwrap();
}

fn _load_precomputed(file_name: String) -> Precomputed {
    let json_string = std::fs::read_to_string(file_name).unwrap();
    from_str(&json_string).unwrap()
}

fn _precompute_move(func: fn(u16) -> u16) -> Vec<u16> {
    let mut precomputed = vec![0; 65536];
    for i in 0..16 {
        for j in 0..16 {
            for k in 0..16 {
                for l in 0..16 {
                    let state =
                        (i as u16) << 0 | (j as u16) << 4 | (k as u16) << 8 | (l as u16) << 12;
                    let left = func(state);
                    precomputed[state as usize] = left;
                }
            }
        }
    }

    precomputed
}

pub fn precompute() {
    let move_left = _precompute_move(move_left);
    let move_right = _precompute_move(move_right);
    let precomputed = Precomputed {
        move_left,
        move_right,
    };
    _save_precomputed("precomputed.json".to_string(), precomputed);
}

pub fn load_precomputed() -> Precomputed {
    _load_precomputed("precomputed.json".to_string())
}

/*
PRECOMPUTED FUNCTIONS
*/

fn _move_state(state: u64, direction: Direction, precomputed: &Precomputed) -> u64 {
    match direction {
        Direction::Left | Direction::Right => {
            let rows = get_rows(state);
            match direction {
                Direction::Left => {
                    precomputed.move_left[rows[0] as usize] as u64
                        | (precomputed.move_left[rows[1] as usize] as u64) << 16
                        | (precomputed.move_left[rows[2] as usize] as u64) << 32
                        | (precomputed.move_left[rows[3] as usize] as u64) << 48
                }
                Direction::Right => {
                    precomputed.move_right[rows[0] as usize] as u64
                        | (precomputed.move_right[rows[1] as usize] as u64) << 16
                        | (precomputed.move_right[rows[2] as usize] as u64) << 32
                        | (precomputed.move_right[rows[3] as usize] as u64) << 48
                }
                _ => unreachable!(),
            }
        }
        Direction::Up | Direction::Down => {
            let cols = get_cols(state);
            match direction {
                Direction::Up => transpose(
                    precomputed.move_left[cols[0] as usize] as u64
                        | (precomputed.move_left[cols[1] as usize] as u64) << 16
                        | (precomputed.move_left[cols[2] as usize] as u64) << 32
                        | (precomputed.move_left[cols[3] as usize] as u64) << 48,
                ),
                Direction::Down => transpose(
                    precomputed.move_right[cols[0] as usize] as u64
                        | (precomputed.move_right[cols[1] as usize] as u64) << 16
                        | (precomputed.move_right[cols[2] as usize] as u64) << 32
                        | (precomputed.move_right[cols[3] as usize] as u64) << 48,
                ),

                _ => unreachable!(),
            }
        }
    }
}

pub fn move_state(
    state: u64,
    direction: Direction,
    add_random: bool,
    precomputed: &Precomputed,
) -> u64 {
    let state = _move_state(state, direction, precomputed);
    if add_random {
        add_random_tile(state)
    } else {
        state
    }
}

pub fn get_possible_moves(state: u64, precomputed: &Precomputed) -> Vec<Direction> {
    let mut moves = Vec::new();
    if move_state(state, Direction::Left, false, precomputed) != state {
        moves.push(Direction::Left);
    }
    if move_state(state, Direction::Right, false, precomputed) != state {
        moves.push(Direction::Right);
    }
    if move_state(state, Direction::Up, false, precomputed) != state {
        moves.push(Direction::Up);
    }
    if move_state(state, Direction::Down, false, precomputed) != state {
        moves.push(Direction::Down);
    }
    moves
}

pub fn is_game_over(state: u64, precomputed: &Precomputed) -> bool {
    if move_state(state, Direction::Left, false, precomputed) != state {
        return false;
    }
    if move_state(state, Direction::Right, false, precomputed) != state {
        return false;
    }
    if move_state(state, Direction::Up, false, precomputed) != state {
        return false;
    }
    if move_state(state, Direction::Down, false, precomputed) != state {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_state() {
        let precomputed: &Precomputed = &load_precomputed();

        let mut grid = [[3, 4, 5, 6], [0, 0, 5, 6], [1, 1, 1, 1], [0, 5, 5, 0]];
        let mut state = from_grid(grid);
        state = move_state(state, Direction::Left, false, precomputed);
        grid = to_grid(state);
        assert_eq!(
            grid,
            [[3, 4, 5, 6], [5, 6, 0, 0], [2, 2, 0, 0], [6, 0, 0, 0]]
        );

        grid = [[1, 1, 0, 0], [0, 2, 0, 2], [3, 3, 4, 0], [0, 0, 0, 0]];
        state = from_grid(grid);
        state = move_state(state, Direction::Right, false, precomputed);
        grid = to_grid(state);
        assert_eq!(
            grid,
            [[0, 0, 0, 2], [0, 0, 0, 3], [0, 0, 4, 4], [0, 0, 0, 0]]
        );

        grid = [[1, 1, 2, 0], [1, 2, 0, 4], [0, 2, 2, 4], [0, 2, 0, 0]];
        state = from_grid(grid);
        state = move_state(state, Direction::Up, false, precomputed);
        grid = to_grid(state);
        assert_eq!(
            grid,
            [[2, 1, 3, 5], [0, 3, 0, 0], [0, 2, 0, 0], [0, 0, 0, 0]]
        );

        grid = [[0, 1, 2, 3], [0, 1, 0, 3], [3, 2, 2, 3], [3, 0, 0, 4]];
        state = from_grid(grid);
        state = move_state(state, Direction::Down, false, precomputed);
        grid = to_grid(state);
        assert_eq!(
            grid,
            [[0, 0, 0, 0], [0, 0, 0, 3], [0, 2, 0, 4], [4, 2, 3, 4]]
        );
    }

    #[test]
    fn test_get_possible_moves() {
        let precomputed: &Precomputed = &load_precomputed();

        let mut grid = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        let mut state = from_grid(grid);
        let possible_moves = get_possible_moves(state, precomputed);
        assert_eq!(possible_moves.len(), 2);
        assert_eq!(possible_moves[0], Direction::Right);
        assert_eq!(possible_moves[1], Direction::Down);

        grid = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]];
        state = from_grid(grid);
        let possible_moves = get_possible_moves(state, precomputed);
        assert_eq!(possible_moves.len(), 4);

        grid = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 1]];
        state = from_grid(grid);
        let possible_moves = get_possible_moves(state, precomputed);
        assert_eq!(possible_moves.len(), 0);
    }

    #[test]
    fn test_is_game_over() {
        let precomputed: &Precomputed = &load_precomputed();

        let mut grid = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        let mut state = from_grid(grid);
        assert_eq!(is_game_over(state, precomputed), false);
        grid = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 1]];
        state = from_grid(grid);
        assert_eq!(is_game_over(state, precomputed), true);
    }
}
