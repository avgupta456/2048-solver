use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use crate::game::{get_cols, get_rows, move_left, move_right, transpose, Direction};

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
    // TODO: document what each of these are
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

pub fn move_state(state: u64, direction: Direction, precomputed: &Precomputed) -> u64 {
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

pub fn get_possible_moves(state: u64, precomputed: &Precomputed) -> Vec<(Direction, u64)> {
    let mut moves = Vec::new();
    let left = move_state(state, Direction::Left, precomputed);
    if left != state {
        moves.push((Direction::Left, left));
    }
    let right = move_state(state, Direction::Right, precomputed);
    if right != state {
        moves.push((Direction::Right, right));
    }
    let up = move_state(state, Direction::Up, precomputed);
    if up != state {
        moves.push((Direction::Up, up));
    }
    let down = move_state(state, Direction::Down, precomputed);
    if down != state {
        moves.push((Direction::Down, down));
    }
    moves
}

pub fn is_game_over(state: u64, precomputed: &Precomputed) -> bool {
    if move_state(state, Direction::Left, precomputed) != state {
        return false;
    }
    if move_state(state, Direction::Right, precomputed) != state {
        return false;
    }
    if move_state(state, Direction::Up, precomputed) != state {
        return false;
    }
    if move_state(state, Direction::Down, precomputed) != state {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{from_grid, to_grid};

    #[test]
    fn test_move_state() {
        let precomputed: &Precomputed = &load_precomputed();

        let mut grid = [[3, 4, 5, 6], [0, 0, 5, 6], [1, 1, 1, 1], [0, 5, 5, 0]];
        let mut state = from_grid(grid);
        state = move_state(state, Direction::Left, precomputed);
        grid = to_grid(state);
        assert_eq!(
            grid,
            [[3, 4, 5, 6], [5, 6, 0, 0], [2, 2, 0, 0], [6, 0, 0, 0]]
        );

        grid = [[1, 1, 0, 0], [0, 2, 0, 2], [3, 3, 4, 0], [0, 0, 0, 0]];
        state = from_grid(grid);
        state = move_state(state, Direction::Right, precomputed);
        grid = to_grid(state);
        assert_eq!(
            grid,
            [[0, 0, 0, 2], [0, 0, 0, 3], [0, 0, 4, 4], [0, 0, 0, 0]]
        );

        grid = [[1, 1, 2, 0], [1, 2, 0, 4], [0, 2, 2, 4], [0, 2, 0, 0]];
        state = from_grid(grid);
        state = move_state(state, Direction::Up, precomputed);
        grid = to_grid(state);
        assert_eq!(
            grid,
            [[2, 1, 3, 5], [0, 3, 0, 0], [0, 2, 0, 0], [0, 0, 0, 0]]
        );

        grid = [[0, 1, 2, 3], [0, 1, 0, 3], [3, 2, 2, 3], [3, 0, 0, 4]];
        state = from_grid(grid);
        state = move_state(state, Direction::Down, precomputed);
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
        // assert first element of tuple is direction::right
        assert_eq!(possible_moves[0].0, Direction::Right);
        assert_eq!(possible_moves[1].0, Direction::Down);

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
