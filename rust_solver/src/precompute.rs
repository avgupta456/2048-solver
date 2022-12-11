use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[allow(unused_imports)]
use crate::game::{move_left, move_right, Direction, State};

/*
PRECOMPUTATION INFRASTRUCTURE
*/

#[derive(Serialize, Deserialize)]
pub struct Precomputed {
    move_left: Vec<[u16; 4]>,
    move_right: Vec<[u16; 4]>,
}

fn _save_precomputed(file_name: String, data: Precomputed) {
    let json_string = to_string(&data).unwrap();
    std::fs::write(file_name, json_string).unwrap();
}

fn _load_precomputed(file_name: String) -> Precomputed {
    let json_string = std::fs::read_to_string(file_name).unwrap();
    from_str(&json_string).unwrap()
}

fn _precompute_move(func: fn([u16; 4]) -> [u16; 4]) -> Vec<[u16; 4]> {
    let mut precomputed = vec![[0; 4]; 65536];
    for i in 0..16 {
        for j in 0..16 {
            for k in 0..16 {
                for l in 0..16 {
                    let result = func([i, j, k, l]);
                    let index = (i << 12) + (j << 8) + (k << 4) + l;
                    precomputed[index as usize] = result;
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

pub fn move_state(state: State, direction: Direction, precomputed: &Precomputed) -> State {
    match direction {
        Direction::Left | Direction::Right => {
            let rows = state.row_ints();
            match direction {
                Direction::Left => State {
                    grid: [
                        precomputed.move_left[rows.0 as usize],
                        precomputed.move_left[rows.1 as usize],
                        precomputed.move_left[rows.2 as usize],
                        precomputed.move_left[rows.3 as usize],
                    ],
                },
                Direction::Right => State {
                    grid: [
                        precomputed.move_right[rows.0 as usize],
                        precomputed.move_right[rows.1 as usize],
                        precomputed.move_right[rows.2 as usize],
                        precomputed.move_right[rows.3 as usize],
                    ],
                },
                _ => unreachable!(),
            }
        }
        Direction::Up | Direction::Down => {
            let cols = state.col_ints();
            match direction {
                Direction::Up => State {
                    grid: [
                        precomputed.move_left[cols.0 as usize],
                        precomputed.move_left[cols.1 as usize],
                        precomputed.move_left[cols.2 as usize],
                        precomputed.move_left[cols.3 as usize],
                    ],
                }
                .transpose(),
                Direction::Down => State {
                    grid: [
                        precomputed.move_right[cols.0 as usize],
                        precomputed.move_right[cols.1 as usize],
                        precomputed.move_right[cols.2 as usize],
                        precomputed.move_right[cols.3 as usize],
                    ],
                }
                .transpose(),
                _ => unreachable!(),
            }
        }
    }
}

pub fn get_possible_moves(state: State, precomputed: &Precomputed) -> Vec<(Direction, State)> {
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

pub fn is_game_over(state: State, precomputed: &Precomputed) -> bool {
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

    #[test]
    fn test_move_state() {
        let precomputed: &Precomputed = &load_precomputed();

        let mut state = State {
            grid: [[3, 4, 5, 6], [0, 0, 5, 6], [1, 1, 1, 1], [0, 5, 5, 0]],
        };
        state = move_state(state, Direction::Left, precomputed);
        assert_eq!(
            state.grid,
            [[3, 4, 5, 6], [5, 6, 0, 0], [2, 2, 0, 0], [6, 0, 0, 0]]
        );

        state.grid = [[1, 1, 0, 0], [0, 2, 0, 2], [3, 3, 4, 0], [0, 0, 0, 0]];
        state = move_state(state, Direction::Right, precomputed);
        assert_eq!(
            state.grid,
            [[0, 0, 0, 2], [0, 0, 0, 3], [0, 0, 4, 4], [0, 0, 0, 0]]
        );

        state.grid = [[1, 1, 2, 0], [1, 2, 0, 4], [0, 2, 2, 4], [0, 2, 0, 0]];
        state = move_state(state, Direction::Up, precomputed);
        assert_eq!(
            state.grid,
            [[2, 1, 3, 5], [0, 3, 0, 0], [0, 2, 0, 0], [0, 0, 0, 0]]
        );

        state.grid = [[0, 1, 2, 3], [0, 1, 0, 3], [3, 2, 2, 3], [3, 0, 0, 4]];
        state = move_state(state, Direction::Down, precomputed);
        assert_eq!(
            state.grid,
            [[0, 0, 0, 0], [0, 0, 0, 3], [0, 2, 0, 4], [4, 2, 3, 4]]
        );
    }

    #[test]
    fn test_get_possible_moves() {
        let precomputed: &Precomputed = &load_precomputed();

        let mut state = State {
            grid: [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        };
        let possible_moves = get_possible_moves(state, precomputed);
        assert_eq!(possible_moves.len(), 2);
        // assert first element of tuple is direction::right
        assert_eq!(possible_moves[0].0, Direction::Right);
        assert_eq!(possible_moves[1].0, Direction::Down);

        state.grid = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]];
        let possible_moves = get_possible_moves(state, precomputed);
        assert_eq!(possible_moves.len(), 4);

        state.grid = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 1]];
        let possible_moves = get_possible_moves(state, precomputed);
        assert_eq!(possible_moves.len(), 0);
    }

    #[test]
    fn test_is_game_over() {
        let precomputed: &Precomputed = &load_precomputed();

        let mut state = State {
            grid: [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        };
        assert_eq!(is_game_over(state, precomputed), false);
        state.grid = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 1]];
        assert_eq!(is_game_over(state, precomputed), true);
    }
}
