use std::cmp;

#[allow(unused_imports)]
use crate::game::{get_empty_tiles, get_score, set_tile, to_grid, Direction};
use crate::precompute::{get_possible_moves, Precomputed};

fn heuristic(state: u64) -> u64 {
    let grid = to_grid(state);

    let pow_grid = [
        [
            2u64.pow(grid[0][0] as u32),
            2u64.pow(grid[0][1] as u32),
            2u64.pow(grid[0][2] as u32),
            2u64.pow(grid[0][3] as u32),
        ],
        [
            2u64.pow(grid[1][0] as u32),
            2u64.pow(grid[1][1] as u32),
            2u64.pow(grid[1][2] as u32),
            2u64.pow(grid[1][3] as u32),
        ],
        [
            2u64.pow(grid[2][0] as u32),
            2u64.pow(grid[2][1] as u32),
            2u64.pow(grid[2][2] as u32),
            2u64.pow(grid[2][3] as u32),
        ],
        [
            2u64.pow(grid[3][0] as u32),
            2u64.pow(grid[3][1] as u32),
            2u64.pow(grid[3][2] as u32),
            2u64.pow(grid[3][3] as u32),
        ],
    ];

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
    state: u64,
    prob: f32,
    depth: u16,
    min_prob: f32,
    precomputed: &Precomputed,
) -> (Direction, f32) {
    let moves = get_possible_moves(state, precomputed);
    if moves.len() == 0 {
        return (Direction::Left, 0.0);
    }

    if depth == 0 {
        return (moves[0].0, heuristic(state) as f32);
    }

    let mut best_move = (moves[0].0, 0.0);
    for (direction, next_state) in moves {
        let empty_tiles = get_empty_tiles(next_state);
        let frac = 1.0 / (empty_tiles.len() as f32);
        let mut next_score = 0.0;
        let mut denom = 0.0;
        for tile in empty_tiles {
            let x = tile % 4;
            let y = tile / 4;
            let temp_state = set_tile(next_state, x, y, 1);
            let _next_score = _get_expectimax_move(
                temp_state,
                prob * frac * 0.9,
                depth - 1,
                min_prob,
                precomputed,
            );
            next_score += frac * 0.9 * _next_score.1;
            denom += frac * 0.9;

            if prob * frac * 0.1 > min_prob {
                let temp_state = set_tile(next_state, x, y, 2);
                let _next_score = _get_expectimax_move(
                    temp_state,
                    prob * frac * 0.1,
                    depth - 1,
                    min_prob,
                    precomputed,
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

    best_move
}

pub fn get_expectimax_move(
    state: u64,
    moves: Vec<(Direction, u64)>,
    depth: u16,
    precomputed: &Precomputed,
) -> (Direction, u64) {
    let min_prob = 0.1 / ((1 << (depth + 4)) as f32);
    let (direction, _) = _get_expectimax_move(state, 1.0, depth, min_prob, precomputed);
    println!("Expectimax move: {:?}", direction);
    let next_state = moves.iter().find(|(dir, _)| *dir == direction).unwrap().1;

    (direction, next_state)
}
