// import rand;
use rand;

fn pow(x: u8) -> u64 {
    2u64.pow(x as u32)
}

fn get_tile(state: u64, x: u8, y: u8) -> u8 {
    let shift = (x + y * 4) * 4;
    ((state >> shift) & 0xf) as u8
}

fn to_grid(state: u64) -> [[u8; 4]; 4] {
    let mut grid = [[0; 4]; 4];
    for y in 0..4 {
        for x in 0..4 {
            grid[y][x] = get_tile(state, x as u8, y as u8);
        }
    }
    grid
}

fn set_tile(state: u64, x: u8, y: u8, value: u8) -> u64 {
    let shift = (x + y * 4) * 4;
    (state & !(0xf << shift)) | ((value as u64) << shift)
}

fn get_row(state: u64, y: u8) -> u64 {
    let shift = y * 4;
    (state >> shift) & 0xffff
}

fn set_row(state: u64, y: u8, value: u64) -> u64 {
    let shift = y * 4;
    (state & !(0xffff << shift)) | (value << shift)
}

fn get_col(state: u64, x: u8) -> u64 {
    let shift = x * 4;
    (state >> shift) & 0x1111111111111111
}

fn set_col(state: u64, x: u8, value: u64) -> u64 {
    let shift = x * 4;
    (state & !(0x1111111111111111 << shift)) | (value << shift)
}

fn get_empty_tiles(state: u64) -> Vec<u8> {
    let mut empty_tiles = Vec::new();
    for y in 0..4 {
        for x in 0..4 {
            if get_tile(state, x, y) == 0 {
                empty_tiles.push(x + y * 4);
            }
        }
    }
    empty_tiles
}

fn add_random_tile(state: u64) -> u64 {
    let empty_tiles = get_empty_tiles(state);
    if empty_tiles.len() == 0 {
        return state;
    }
    let index = rand::random::<usize>() % empty_tiles.len();
    let tile = empty_tiles[index];
    let x = tile % 4;
    let y = tile / 4;
    let value = if rand::random::<f32>() < 0.9 { 1 } else { 2 };
    set_tile(state, x, y, value)
}

pub fn get_initial_state() -> u64 {
    let mut state = 0;
    state = add_random_tile(state);
    state = add_random_tile(state);
    state
}

fn merge(row: u64) -> u64 {
    let mut result = 0;
    let mut last = 0;
    let mut last_value = 0;
    for i in 0..4 {
        let value = ((row >> (i * 4)) & 0xf) as u8;
        if value == 0 {
            continue;
        }
        if value == last_value {
            result |= ((last_value + 1) as u64) << (last * 4);
            last_value = 0;
        } else {
            if last_value != 0 {
                result |= (last_value as u64) << (last * 4);
            }
            last = i;
            last_value = value;
        }
    }
    if last_value != 0 {
        result |= (last_value as u64) << (last * 4);
    }
    result
}

fn move_left(state: u64) -> u64 {
    let mut result = 0;
    for y in 0..4 {
        let row = get_row(state, y);
        let merged = merge(row);
        result = set_row(result, y, merged);
    }
    result
}

fn move_right(state: u64) -> u64 {
    let mut result = 0;
    for y in 0..4 {
        let row = get_row(state, y);
        let merged = merge(row.reverse_bits() >> 48);
        result = set_row(result, y, merged.reverse_bits() >> 48);
    }
    result
}

fn move_up(state: u64) -> u64 {
    let mut result = 0;
    for x in 0..4 {
        let col = get_col(state, x);
        let merged = merge(col);
        result = set_col(result, x, merged);
    }
    result
}

fn move_down(state: u64) -> u64 {
    let mut result = 0;
    for x in 0..4 {
        let col = get_col(state, x);
        let merged = merge(col.reverse_bits() >> 48);
        result = set_col(result, x, merged.reverse_bits() >> 48);
    }
    result
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn get_possible_moves(state: u64) -> Vec<Direction> {
    let mut moves = Vec::new();
    if move_left(state) != state {
        moves.push(Direction::Left);
    }
    if move_right(state) != state {
        moves.push(Direction::Right);
    }
    if move_up(state) != state {
        moves.push(Direction::Up);
    }
    if move_down(state) != state {
        moves.push(Direction::Down);
    }
    moves
}

pub fn move_state(state: u64, direction: Direction, add_random: bool) -> u64 {
    let state = match direction {
        Direction::Left => move_left(state),
        Direction::Right => move_right(state),
        Direction::Up => move_up(state),
        Direction::Down => move_down(state),
    };

    if add_random {
        add_random_tile(state)
    } else {
        state
    }
}

pub fn is_game_over(state: u64) -> bool {
    get_possible_moves(state).len() == 0
}

fn get_score(state: u64) -> u64 {
    let mut score = 0;
    for y in 0..4 {
        for x in 0..4 {
            let value = get_tile(state, x, y);
            if value > 0 {
                score += ((value - 1) as u64) * pow(value);
            }
        }
    }
    score
}

pub fn print_board(state: u64) {
    for y in 0..4 {
        for x in 0..4 {
            let value = get_tile(state, x, y);
            // print num or . padded with tab
            if value == 0 {
                print!(".\t");
            } else {
                print!("{}\t", pow(value));
            }
        }
        println!();
    }
    println!();
    println!("Score: {}", get_score(state));
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tile() {
        let state = 0x123456789abcdef0;
        assert_eq!(get_tile(state, 0, 0), 0);
        assert_eq!(get_tile(state, 1, 0), 15);
        assert_eq!(get_tile(state, 2, 0), 14);
        assert_eq!(get_tile(state, 3, 0), 13);
        assert_eq!(get_tile(state, 0, 1), 12);
        assert_eq!(get_tile(state, 1, 1), 11);
        assert_eq!(get_tile(state, 2, 1), 10);
        assert_eq!(get_tile(state, 3, 1), 9);
        assert_eq!(get_tile(state, 0, 2), 8);
        assert_eq!(get_tile(state, 1, 2), 7);
        assert_eq!(get_tile(state, 2, 2), 6);
        assert_eq!(get_tile(state, 3, 2), 5);
        assert_eq!(get_tile(state, 0, 3), 4);
        assert_eq!(get_tile(state, 1, 3), 3);
        assert_eq!(get_tile(state, 2, 3), 2);
        assert_eq!(get_tile(state, 3, 3), 1);
    }

    #[test]
    fn test_to_grid() {
        let state = 0x123456789abcdef0;
        let grid = to_grid(state);
        assert_eq!(grid[0][0], 0);
        assert_eq!(grid[0][1], 15);
        assert_eq!(grid[0][2], 14);
        assert_eq!(grid[0][3], 13);
        assert_eq!(grid[1][0], 12);
        assert_eq!(grid[1][1], 11);
        assert_eq!(grid[1][2], 10);
        assert_eq!(grid[1][3], 9);
        assert_eq!(grid[2][0], 8);
        assert_eq!(grid[2][1], 7);
        assert_eq!(grid[2][2], 6);
        assert_eq!(grid[2][3], 5);
        assert_eq!(grid[3][0], 4);
        assert_eq!(grid[3][1], 3);
        assert_eq!(grid[3][2], 2);
        assert_eq!(grid[3][3], 1);
    }

    #[test]
    fn test_set_tile() {
        let state = 0x123456789abcdef0;
        assert_eq!(to_grid(set_tile(state, 0, 0, 0))[0][0], 0);
        assert_eq!(to_grid(set_tile(state, 1, 0, 0))[0][1], 0);
        assert_eq!(to_grid(set_tile(state, 2, 0, 0))[0][2], 0);
        assert_eq!(to_grid(set_tile(state, 3, 0, 0))[0][3], 0);
        assert_eq!(to_grid(set_tile(state, 0, 1, 0))[1][0], 0);
        assert_eq!(to_grid(set_tile(state, 1, 1, 0))[1][1], 0);
        assert_eq!(to_grid(set_tile(state, 2, 1, 0))[1][2], 0);
        assert_eq!(to_grid(set_tile(state, 3, 1, 0))[1][3], 0);
        assert_eq!(to_grid(set_tile(state, 0, 2, 0))[2][0], 0);
        assert_eq!(to_grid(set_tile(state, 1, 2, 0))[2][1], 0);
        assert_eq!(to_grid(set_tile(state, 2, 2, 0))[2][2], 0);
        assert_eq!(to_grid(set_tile(state, 3, 2, 0))[2][3], 0);
        assert_eq!(to_grid(set_tile(state, 0, 3, 0))[3][0], 0);
        assert_eq!(to_grid(set_tile(state, 1, 3, 0))[3][1], 0);
        assert_eq!(to_grid(set_tile(state, 2, 3, 0))[3][2], 0);
        assert_eq!(to_grid(set_tile(state, 3, 3, 0))[3][3], 0);
    }
}
