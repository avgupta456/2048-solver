use std::cmp;

use super::game::{Direction, State};
use super::precompute::{get_possible_moves, Precomputed, TranspositionTable};

// TODO: Precompute this
fn pow_grid(state: &State) -> [[u64; 4]; 4] {
    [
        [
            2u64.pow(state.grid[0][0] as u32),
            2u64.pow(state.grid[0][1] as u32),
            2u64.pow(state.grid[0][2] as u32),
            2u64.pow(state.grid[0][3] as u32),
        ],
        [
            2u64.pow(state.grid[1][0] as u32),
            2u64.pow(state.grid[1][1] as u32),
            2u64.pow(state.grid[1][2] as u32),
            2u64.pow(state.grid[1][3] as u32),
        ],
        [
            2u64.pow(state.grid[2][0] as u32),
            2u64.pow(state.grid[2][1] as u32),
            2u64.pow(state.grid[2][2] as u32),
            2u64.pow(state.grid[2][3] as u32),
        ],
        [
            2u64.pow(state.grid[3][0] as u32),
            2u64.pow(state.grid[3][1] as u32),
            2u64.pow(state.grid[3][2] as u32),
            2u64.pow(state.grid[3][3] as u32),
        ],
    ]
}

fn corner_heuristic(state: State) -> u64 {
    let pow_grid = pow_grid(&state);

    let lower_left = 10 * pow_grid[3][0]
        + 5 * pow_grid[2][0]
        + 2 * pow_grid[1][0]
        + pow_grid[0][0]
        + 5 * pow_grid[3][1]
        + 2 * pow_grid[2][1]
        + pow_grid[1][1]
        + 2 * pow_grid[3][2]
        + pow_grid[2][2]
        + pow_grid[3][3];
    let lower_right = 10 * pow_grid[3][3]
        + 5 * pow_grid[2][3]
        + 2 * pow_grid[1][3]
        + pow_grid[0][3]
        + 5 * pow_grid[3][2]
        + 2 * pow_grid[2][2]
        + pow_grid[1][2]
        + 2 * pow_grid[3][1]
        + pow_grid[2][1]
        + pow_grid[3][0];
    let upper_left = 10 * pow_grid[0][0]
        + 5 * pow_grid[1][0]
        + 2 * pow_grid[2][0]
        + pow_grid[3][0]
        + 5 * pow_grid[0][1]
        + 2 * pow_grid[1][1]
        + pow_grid[2][1]
        + 2 * pow_grid[0][2]
        + pow_grid[1][2]
        + pow_grid[0][3];
    let upper_right = 10 * pow_grid[0][3]
        + 5 * pow_grid[1][3]
        + 2 * pow_grid[2][3]
        + pow_grid[3][3]
        + 5 * pow_grid[0][2]
        + 2 * pow_grid[1][2]
        + pow_grid[2][2]
        + 2 * pow_grid[0][1]
        + pow_grid[1][1]
        + pow_grid[0][0];

    cmp::max(
        cmp::max(lower_left, lower_right),
        cmp::max(upper_left, upper_right),
    )
}

fn _get_expectimax_move(
    state: State,
    prob: f32,
    depth: u16,
    min_prob: f32,
    precomputed: &Precomputed,
    transposition: &mut TranspositionTable,
) -> (Direction, f32) {
    let moves = get_possible_moves(state, precomputed);
    if moves[0].0 == Direction::Invalid {
        return (Direction::Invalid, 0.0);
    }

    if depth == 0 {
        return (moves[0].0, corner_heuristic(state) as f32);
    }

    let lookup = transposition.get(&state, depth, prob);
    if let Some((direction, score)) = lookup {
        return (*direction, *score);
    }

    let mut best_move = (Direction::Invalid, -1.0);
    for (direction, next_state) in moves {
        if direction == Direction::Invalid {
            continue;
        }

        let empty_tiles = next_state.get_empty_tiles();
        let frac = 1.0 / (empty_tiles.len() as f32);
        let mut next_score = 0.0;
        let mut denom = 0.0;
        for (x, y) in empty_tiles {
            let mut temp_state = next_state;
            temp_state.grid[y as usize][x as usize] = 1;
            let _next_score = _get_expectimax_move(
                temp_state,
                prob * frac * 0.9,
                depth - 1,
                min_prob,
                precomputed,
                transposition,
            );
            next_score += frac * 0.9 * _next_score.1;
            denom += frac * 0.9;

            if prob * frac * 0.1 > min_prob {
                let mut temp_state = next_state;
                temp_state.grid[y as usize][x as usize] = 2;
                let _next_score = _get_expectimax_move(
                    temp_state,
                    prob * frac * 0.1,
                    depth - 1,
                    min_prob,
                    precomputed,
                    transposition,
                );
                next_score += frac * 0.1 * _next_score.1;
                denom += frac * 0.1;
            }
        }
        next_score /= denom;
        if next_score > best_move.1 {
            best_move = (direction, next_score);
        }
    }

    transposition.insert(state, depth, prob, best_move);

    best_move
}

pub fn get_expectimax_move(
    state: State,
    moves: [(Direction, State); 4],
    depth: u16,
    precomputed: &Precomputed,
    transposition: &mut TranspositionTable,
) -> (Direction, State) {
    let min_prob = 0.1 / ((1 << (depth + 4)) as f32);
    let (direction, _) =
        _get_expectimax_move(state, 1.0, depth, min_prob, precomputed, transposition);
    let next_state = moves.iter().find(|(dir, _)| *dir == direction).unwrap().1;
    transposition.clear();
    (direction, next_state)
}
