use rand;

/*
ENUMS
*/

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/*
LOW-LEVEL BOARD MANIPULATION
*/

fn get_tile(state: u64, x: u8, y: u8) -> u8 {
    let shift = (x + y * 4) * 4;
    ((state >> shift) & 0xf) as u8
}

pub fn to_grid(state: u64) -> [[u8; 4]; 4] {
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

pub fn from_grid(grid: [[u8; 4]; 4]) -> u64 {
    let mut state = 0;
    for y in 0..4 {
        for x in 0..4 {
            state = set_tile(state, x as u8, y as u8, grid[y][x]);
        }
    }
    state
}

// From https://github.com/aszczepanski/2048
pub fn transpose(state: u64) -> u64 {
    let a1 = state & 0xF0F00F0FF0F00F0F;
    let a2 = state & 0x0000F0F00000F0F0;
    let a3 = state & 0x0F0F00000F0F0000;
    let a = a1 | (a2 << 12) | (a3 >> 12);
    let b1 = a & 0xFF00FF0000FF00FF;
    let b2 = a & 0x00FF00FF00000000;
    let b3 = a & 0x00000000FF00FF00;
    b1 | (b2 >> 24) | (b3 << 24)
}

pub fn get_rows(state: u64) -> Vec<u16> {
    let mut rows = Vec::new();
    for y in 0..4 {
        rows.push(((state >> (y * 16)) & 0xffff) as u16);
    }
    rows
}

pub fn set_rows(rows: Vec<u16>) -> u64 {
    let mut state = 0;
    for y in 0..4 {
        state |= (rows[y] as u64) << (y * 16);
    }
    state
}

pub fn get_cols(state: u64) -> Vec<u16> {
    let t = transpose(state);
    get_rows(t)
}

pub fn set_cols(cols: Vec<u16>) -> u64 {
    let t = set_rows(cols);
    transpose(t)
}

/*
BOARD OPERATIONS, NOT PRECOMPUTED
*/

// TODO: Move to precomputed
pub fn get_empty_tiles(state: u64) -> Vec<u8> {
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

pub fn add_random_tile(state: u64) -> u64 {
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

/*
GETS PRECOMPUTED, DO NOT CALL DIRECTLY
*/

fn merge(row: u16) -> u16 {
    let mut arr: Vec<u8> = Vec::new();
    for i in 0..4 {
        let value = ((row >> (i * 4)) & 0xf) as u8;
        if value == 0 {
            continue;
        }
        if arr.len() > 0 && arr[arr.len() - 1] == value {
            let prev = arr.pop().unwrap();
            arr.push(prev + 1);
        } else {
            arr.push(value);
        }
    }
    for _ in arr.len()..4 {
        arr.push(0);
    }
    (arr[0] as u16) | ((arr[1] as u16) << 4) | ((arr[2] as u16) << 8) | ((arr[3] as u16) << 12)
}

fn reverse(row: u16) -> u16 {
    ((row & 0xf) << 12) | ((row & 0xf0) << 4) | ((row & 0xf00) >> 4) | ((row & 0xf000) >> 12)
}

pub fn move_left(row: u16) -> u16 {
    merge(row)
}

pub fn move_right(row: u16) -> u16 {
    reverse(merge(reverse(row)))
}

/*
HELPER FUNCTIONS
*/

pub fn get_score(state: u64) -> u64 {
    let mut score = 0;
    for y in 0..4 {
        for x in 0..4 {
            let value = get_tile(state, x, y);
            if value > 0 {
                score += ((value - 1) as u64) * 2u64.pow(value as u32);
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
                print!("{}\t", 2u64.pow(value as u32));
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

    #[test]
    fn test_from_grid() {
        let grid = [[0, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]];
        let state = from_grid(grid);
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
    fn test_transpose() {
        let grid = [[0, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]];
        let transposed = to_grid(transpose(from_grid(grid)));

        assert_eq!(transposed[0][0], 0);
        assert_eq!(transposed[0][1], 12);
        assert_eq!(transposed[0][2], 8);
        assert_eq!(transposed[0][3], 4);
        assert_eq!(transposed[1][0], 15);
        assert_eq!(transposed[1][1], 11);
        assert_eq!(transposed[1][2], 7);
        assert_eq!(transposed[1][3], 3);
        assert_eq!(transposed[2][0], 14);
        assert_eq!(transposed[2][1], 10);
        assert_eq!(transposed[2][2], 6);
        assert_eq!(transposed[2][3], 2);
        assert_eq!(transposed[3][0], 13);
        assert_eq!(transposed[3][1], 9);
        assert_eq!(transposed[3][2], 5);
        assert_eq!(transposed[3][3], 1);
    }

    #[test]
    fn test_get_rows() {
        let grid = [[0, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]];
        let rows = get_rows(from_grid(grid));

        assert_eq!(rows[0], 0xdef0);
        assert_eq!(rows[1], 0x9abc);
        assert_eq!(rows[2], 0x5678);
        assert_eq!(rows[3], 0x1234);
    }

    #[test]
    fn test_get_cols() {
        let grid = [[0, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]];
        let cols = get_cols(from_grid(grid));

        assert_eq!(cols[0], 0x48c0);
        assert_eq!(cols[1], 0x37bf);
        assert_eq!(cols[2], 0x26ae);
        assert_eq!(cols[3], 0x159d);
    }

    #[test]
    fn test_set_rows() {
        let grid = [[0, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]];
        let state = from_grid(grid);
        let rows = get_rows(state);
        let new_state = set_rows(rows);

        assert_eq!(state, new_state);
    }

    #[test]
    fn test_set_cols() {
        let grid = [[0, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]];
        let state = from_grid(grid);
        let cols = get_cols(state);
        let new_state = set_cols(cols);

        assert_eq!(state, new_state);
    }

    #[test]
    fn test_get_empty_tiles() {
        let grid = [[0, 15, 14, 13], [12, 11, 0, 9], [8, 7, 6, 5], [4, 3, 2, 1]];
        let state = from_grid(grid);
        let empty_tiles = get_empty_tiles(state);

        assert_eq!(empty_tiles.len(), 2);
        assert_eq!(empty_tiles[0], 0);
        assert_eq!(empty_tiles[1], 6);
    }

    #[test]
    fn test_add_random_tile() {
        let mut state = 0;
        state = add_random_tile(state);
        assert_eq!(get_empty_tiles(state).len(), 15);
        state = add_random_tile(state);
        assert_eq!(get_empty_tiles(state).len(), 14);
    }

    #[test]
    fn test_get_initial_state() {
        let state = get_initial_state();
        assert_eq!(get_empty_tiles(state).len(), 14);
        let grid = to_grid(state);
        let max_tile = grid.iter().flat_map(|row| row.iter()).max().unwrap();
        assert_eq!(*max_tile == 1 || *max_tile == 2, true); // before pow
    }

    #[test]
    fn test_merge() {
        let mut row = 0x0000;
        row = merge(row);
        assert_eq!(row, 0x0000);
        row = 0x0001;
        row = merge(row);
        assert_eq!(row, 0x0001);
        row = 0x0010;
        row = merge(row);
        assert_eq!(row, 0x0001);
        row = 0x0230;
        row = merge(row);
        assert_eq!(row, 0x0023);
        row = 0x0220;
        row = merge(row);
        assert_eq!(row, 0x0003);
        row = 0x2220;
        row = merge(row);
        assert_eq!(row, 0x0023);
        row = 0x2222;
        row = merge(row);
        assert_eq!(row, 0x0033);
        row = 0x2332;
        row = merge(row);
        assert_eq!(row, 0x0242);
        row = 0x2200;
        row = merge(row);
        assert_eq!(row, 0x0003);
        row = 0x2202;
        row = merge(row);
        assert_eq!(row, 0x0023);
        row = 0x2230;
        row = merge(row);
        assert_eq!(row, 0x0033);
    }

    #[test]
    fn test_reverse() {
        let mut row = 0x0000;
        row = reverse(row);
        assert_eq!(row, 0x0000);
        row = 0x0001;
        row = reverse(row);
        assert_eq!(row, 0x1000);
        row = 0x1234;
        row = reverse(row);
        assert_eq!(row, 0x4321);
    }

    // TODO: test_move_left and test_move_right

    #[test]
    fn test_get_score() {
        let mut grid = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        let mut state = from_grid(grid);
        assert_eq!(get_score(state), 0);
        grid = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 1]];
        state = from_grid(grid);
        assert_eq!(get_score(state), 851972);
    }
}
