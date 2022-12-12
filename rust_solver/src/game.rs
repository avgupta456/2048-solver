use rand;
use serde::{Deserialize, Serialize};

/*
ENUMS
*/

// NOTE: Must index state.grid[y][x]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct State {
    pub grid: [[u16; 4]; 4],
}

impl State {
    pub fn index(&self, x: u16, y: u16) -> u16 {
        self.grid[y as usize][x as usize]
    }

    pub fn row_ints(&self) -> (u16, u16, u16, u16) {
        (
            (self.grid[0][0] << 12)
                | (self.grid[0][1] << 8)
                | self.grid[0][2] << 4
                | self.grid[0][3],
            (self.grid[1][0] << 12)
                | (self.grid[1][1] << 8)
                | self.grid[1][2] << 4
                | self.grid[1][3],
            (self.grid[2][0] << 12)
                | (self.grid[2][1] << 8)
                | self.grid[2][2] << 4
                | self.grid[2][3],
            (self.grid[3][0] << 12)
                | (self.grid[3][1] << 8)
                | self.grid[3][2] << 4
                | self.grid[3][3],
        )
    }

    pub fn col_ints(&self) -> (u16, u16, u16, u16) {
        (
            (self.grid[0][0] << 12)
                | (self.grid[1][0] << 8)
                | self.grid[2][0] << 4
                | self.grid[3][0],
            (self.grid[0][1] << 12)
                | (self.grid[1][1] << 8)
                | self.grid[2][1] << 4
                | self.grid[3][1],
            (self.grid[0][2] << 12)
                | (self.grid[1][2] << 8)
                | self.grid[2][2] << 4
                | self.grid[3][2],
            (self.grid[0][3] << 12)
                | (self.grid[1][3] << 8)
                | self.grid[2][3] << 4
                | self.grid[3][3],
        )
    }

    pub fn transpose(&self) -> State {
        let mut new_state = State { grid: [[0; 4]; 4] };
        for y in 0..4 {
            for x in 0..4 {
                new_state.grid[x][y] = self.grid[y][x];
            }
        }
        new_state
    }

    pub fn get_empty_tiles(&self) -> Vec<(u16, u16)> {
        let mut empty_tiles = Vec::new();
        for y in 0..4 {
            for x in 0..4 {
                if self.grid[y as usize][x as usize] == 0 {
                    empty_tiles.push((x, y));
                }
            }
        }
        empty_tiles
    }

    pub fn add_random_tile(&self) -> State {
        let empty_tiles = self.get_empty_tiles();
        if empty_tiles.len() == 0 {
            return *self;
        }
        let index = rand::random::<usize>() % empty_tiles.len();
        let (x, y) = empty_tiles[index];
        let value = if rand::random::<f32>() < 0.9 { 1 } else { 2 };
        let mut new_state = *self;
        new_state.grid[y as usize][x as usize] = value;
        new_state
    }

    pub fn new() -> State {
        let mut state = State { grid: [[0; 4]; 4] };
        state = state.add_random_tile();
        state = state.add_random_tile();
        state
    }

    pub fn get_score(&self) -> u64 {
        let mut score = 0;
        for y in 0..4 {
            for x in 0..4 {
                let value = self.grid[x as usize][y as usize];
                if value > 0 {
                    score += ((value - 1) as u64) * 2u64.pow(value as u32);
                }
            }
        }
        score
    }

    pub fn print_board(&self) {
        for y in 0..4 {
            for x in 0..4 {
                let value = self.grid[x as usize][y as usize];
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
        println!("Score: {}", self.get_score());
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Invalid,
}

/*
GETS PRECOMPUTED, DO NOT CALL DIRECTLY
*/

fn merge(row: [u16; 4]) -> [u16; 4] {
    let mut arr: Vec<u16> = Vec::new();
    for i in 0..4 {
        let value = row[i];
        if value == 0 {
            continue;
        }
        if arr.len() > 0 && arr[arr.len() - 1] == value {
            let last = arr.pop().unwrap();
            arr.push(last + 1);
            arr.push(100); // prevent double merge
        } else {
            arr.push(value);
        }
    }
    arr = arr.into_iter().filter(|x| *x != 100).collect();
    for _ in arr.len()..4 {
        arr.push(0);
    }
    [arr[0], arr[1], arr[2], arr[3]]
}

pub fn reverse(row: [u16; 4]) -> [u16; 4] {
    [row[3], row[2], row[1], row[0]]
}

pub fn move_left(row: [u16; 4]) -> [u16; 4] {
    merge(row)
}

pub fn move_right(row: [u16; 4]) -> [u16; 4] {
    reverse(merge(reverse(row)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tile() {
        let state = State {
            grid: [[0, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]],
        };
        assert_eq!(state.grid[0][0], 0);
        assert_eq!(state.grid[0][1], 15);
        assert_eq!(state.grid[0][2], 14);
        assert_eq!(state.grid[0][3], 13);
        assert_eq!(state.grid[1][0], 12);
        assert_eq!(state.grid[1][1], 11);
        assert_eq!(state.grid[1][2], 10);
        assert_eq!(state.grid[1][3], 9);
        assert_eq!(state.grid[2][0], 8);
        assert_eq!(state.grid[2][1], 7);
        assert_eq!(state.grid[2][2], 6);
        assert_eq!(state.grid[2][3], 5);
        assert_eq!(state.grid[3][0], 4);
        assert_eq!(state.grid[3][1], 3);
        assert_eq!(state.grid[3][2], 2);
        assert_eq!(state.grid[3][3], 1);
    }

    #[test]
    fn test_transpose() {
        let state = State {
            grid: [[0, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]],
        };
        let transposed = state.transpose();
        assert_eq!(transposed.grid[0][0], 0);
        assert_eq!(transposed.grid[0][1], 12);
        assert_eq!(transposed.grid[0][2], 8);
        assert_eq!(transposed.grid[0][3], 4);
        assert_eq!(transposed.grid[1][0], 15);
        assert_eq!(transposed.grid[1][1], 11);
        assert_eq!(transposed.grid[1][2], 7);
        assert_eq!(transposed.grid[1][3], 3);
        assert_eq!(transposed.grid[2][0], 14);
        assert_eq!(transposed.grid[2][1], 10);
        assert_eq!(transposed.grid[2][2], 6);
        assert_eq!(transposed.grid[2][3], 2);
        assert_eq!(transposed.grid[3][0], 13);
        assert_eq!(transposed.grid[3][1], 9);
        assert_eq!(transposed.grid[3][2], 5);
        assert_eq!(transposed.grid[3][3], 1);
    }

    #[test]
    fn test_get_empty_tiles() {
        let state = State {
            grid: [[0, 15, 14, 13], [12, 11, 0, 9], [8, 7, 6, 5], [4, 3, 2, 1]],
        };
        let empty_tiles = state.get_empty_tiles();
        assert_eq!(empty_tiles.len(), 2);
        assert_eq!(empty_tiles[0], (0, 0));
        assert_eq!(empty_tiles[1], (2, 1));
    }

    #[test]
    fn test_add_random_tile() {
        let mut state = State {
            grid: [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        };
        assert_eq!(state.get_empty_tiles().len(), 16);
        state = state.add_random_tile();
        assert_eq!(state.get_empty_tiles().len(), 15);
        state = state.add_random_tile();
        assert_eq!(state.get_empty_tiles().len(), 14);
    }

    #[test]
    fn test_get_initial_state() {
        let state = State::new();
        assert_eq!(state.get_empty_tiles().len(), 14);
        let max_tile = state.grid.iter().flat_map(|row| row.iter()).max().unwrap();
        assert_eq!(*max_tile == 1 || *max_tile == 2, true); // before pow
    }

    #[test]
    fn test_get_score() {
        let state = State {
            grid: [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        };
        assert_eq!(state.get_score(), 0);
        let state = State {
            grid: [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]],
        };
        assert_eq!(state.get_score(), 851972);
    }

    #[test]
    fn test_merge() {
        let mut row = [0, 0, 0, 0];
        row = merge(row);
        assert_eq!(row, [0, 0, 0, 0]);
        row = [1, 0, 0, 0];
        row = merge(row);
        assert_eq!(row, [1, 0, 0, 0]);
        row = [0, 0, 1, 0];
        row = merge(row);
        assert_eq!(row, [1, 0, 0, 0]);
        row = [0, 2, 3, 0];
        row = merge(row);
        assert_eq!(row, [2, 3, 0, 0]);
        row = [0, 2, 2, 0];
        row = merge(row);
        assert_eq!(row, [3, 0, 0, 0]);
        row = [0, 2, 2, 2];
        row = merge(row);
        assert_eq!(row, [3, 2, 0, 0]);
        row = [2, 2, 2, 2];
        row = merge(row);
        assert_eq!(row, [3, 3, 0, 0]);
        row = [2, 3, 3, 2];
        row = merge(row);
        assert_eq!(row, [2, 4, 2, 0]);
        row = [2, 2, 0, 0];
        row = merge(row);
        assert_eq!(row, [3, 0, 0, 0]);
        row = [2, 2, 0, 2];
        row = merge(row);
        assert_eq!(row, [3, 2, 0, 0]);
        row = [0, 3, 2, 2];
        row = merge(row);
        assert_eq!(row, [3, 3, 0, 0]);
        row = [3, 3, 4, 0];
        row = merge(row);
        assert_eq!(row, [4, 4, 0, 0]);
    }

    #[test]
    fn test_reverse() {
        let row = [1, 2, 3, 4];
        assert_eq!(reverse(row), [4, 3, 2, 1]);
    }

    // TODO: test_move_left and test_move_right
}
