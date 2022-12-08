from functools import lru_cache
from typing import List, Tuple
import random

# Represent 2048 Game state as a single integer, 16 bits per tile


# Game is (state (64 bit int), score)
Game = Tuple[int, int]


def get_tile(state: int, tile: int) -> int:
    # Return the value of a tile
    return (state >> (tile * 4)) & 0xF


@lru_cache(maxsize=2**16)
def exp_tile(tile: int) -> int:
    # Return the value of a tile
    return 2 ** tile


def get_grid(state: int, exp:bool = False, flatten:bool = False) -> List[List[int]]:
    tile1 = get_tile(state, 0)
    tile2 = get_tile(state, 1)
    tile3 = get_tile(state, 2)
    tile4 = get_tile(state, 3)
    tile5 = get_tile(state, 4)
    tile6 = get_tile(state, 5)
    tile7 = get_tile(state, 6)
    tile8 = get_tile(state, 7)
    tile9 = get_tile(state, 8)
    tile10 = get_tile(state, 9)
    tile11 = get_tile(state, 10)
    tile12 = get_tile(state, 11)
    tile13 = get_tile(state, 12)
    tile14 = get_tile(state, 13)
    tile15 = get_tile(state, 14)
    tile16 = get_tile(state, 15)

    if exp:
        tile1 = exp_tile(tile1)
        tile2 = exp_tile(tile2)
        tile3 = exp_tile(tile3)
        tile4 = exp_tile(tile4)
        tile5 = exp_tile(tile5)
        tile6 = exp_tile(tile6)
        tile7 = exp_tile(tile7)
        tile8 = exp_tile(tile8)
        tile9 = exp_tile(tile9)
        tile10 = exp_tile(tile10)
        tile11 = exp_tile(tile11)
        tile12 = exp_tile(tile12)
        tile13 = exp_tile(tile13)
        tile14 = exp_tile(tile14)
        tile15 = exp_tile(tile15)
        tile16 = exp_tile(tile16)

    if flatten:
        return [tile1, tile2, tile3, tile4, tile5, tile6, tile7, tile8, tile9, tile10, tile11, tile12, tile13, tile14, tile15, tile16]
    
    return [
        [tile1, tile2, tile3, tile4],
        [tile5, tile6, tile7, tile8],
        [tile9, tile10, tile11, tile12],
        [tile13, tile14, tile15, tile16]
    ]


def set_tile(state: int, tile: int, value: int) -> int:
    # Set the value of a tile
    return state | (value << (tile * 4))


def get_empty_tiles(state: int) -> List[int]:
    # Return a list of empty tiles
    return [i for i in range(16) if get_tile(state, i) == 0]


def add_random_tile(state: int) -> int:
    empty_tiles = get_empty_tiles(state)
    if empty_tiles:
        tile = random.choice(empty_tiles)
        value = 1 if random.random() < 0.9 else 2
        return set_tile(state, tile, value)
    return state

def get_initial_state() -> int:
    return add_random_tile(add_random_tile(0))


def merge(arr: List[int], reverse=False) -> Tuple[List]:
    # Merge tiles in a row or column
    if reverse:
        arr = arr[::-1]
    merged = []
    for i in range(4):
        if arr[i] == 0:
            continue
        if merged and merged[-1] == arr[i]:
            merged[-1] += 1
        else:
            merged.append(arr[i])
    merged += [0] * (4 - len(merged))
    if reverse:
        merged = merged[::-1]
    return merged


def move_left(state: int):
    # Move all tiles to the left
    new_state = 0
    for i in range(4):
        row = [get_tile(state, i * 4 + j) for j in range(4)]
        row = merge(row)
        for j in range(4):
            new_state = set_tile(new_state, i * 4 + j, row[j])
    return new_state

def move_right(state: int):
    # Move all tiles to the right
    new_state = 0
    for i in range(4):
        row = [get_tile(state, i * 4 + j) for j in range(4)]
        row = merge(row, reverse=True)
        for j in range(4):
            new_state = set_tile(new_state, i * 4 + j, row[j])
    return new_state


def move_up(state: int):
    # Move all tiles up
    new_state = 0
    for j in range(4):
        col = [get_tile(state, i * 4 + j) for i in range(4)]
        col = merge(col)
        for i in range(4):
            new_state = set_tile(new_state, i * 4 + j, col[i])
    return new_state


def move_down(state: int):
    # Move all tiles down
    new_state = 0
    for j in range(4):
        col = [get_tile(state, i * 4 + j) for i in range(4)]
        col = merge(col, reverse=True)
        for i in range(4):
            new_state = set_tile(new_state, i * 4 + j, col[i])
    return new_state


def get_possible_moves(state: int):
    # Return a list of possible moves
    moves = []
    if move_left(state) != state:
        moves.append('left')
    if move_right(state) != state:
        moves.append('right')
    if move_up(state) != state:
        moves.append('up')
    if move_down(state) != state:
        moves.append('down')
    return moves

def make_move(state: int, move: str, add_tile=True):
    # Make a move
    if move == 'left':
        state = move_left(state)
    elif move == 'right':
        state = move_right(state)
    elif move == 'up':
        state = move_up(state)
    elif move == 'down':
        state = move_down(state)
    
    if add_tile:
        state = add_random_tile(state)

    return state


def is_game_over(state: int):
    # Return True if the game is over
    return not get_possible_moves(state)


def get_score(state: int):
    # Return the score
    grid = get_grid(state, flatten=True)
    nonzero = [x for x in grid if x > 0]
    return sum((x - 1) * 2 ** x for x in nonzero)


def print_board(state):
    # Print the board
    for i in range(4):
        for j in range(4):
            tile = get_tile(state, i * 4 + j)
            print(2 ** tile if tile else '.', end='\t')
        print()
    print()
    print('Score:', get_score(state))
    print()
