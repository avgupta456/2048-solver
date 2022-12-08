import random

# Represent 2048 Game state as a single integer, 16 bits per tile


def get_tile(state, tile):
    # Return the value of a tile
    return (state >> (tile * 4)) & 0xF


def set_tile(state, tile, value):
    # Set the value of a tile
    return state | (value << (tile * 4))


def get_empty_tiles(state):
    # Return a list of empty tiles
    return [i for i in range(16) if get_tile(state, i) == 0]


def add_random_tile(state):
    empty_tiles = get_empty_tiles(state)
    if empty_tiles:
        tile = random.choice(empty_tiles)
        value = 1 if random.random() < 0.9 else 2
        return set_tile(state, tile, value)
    return state

def get_initial_state():
    return add_random_tile(add_random_tile(0))


def merge(arr, reverse=False):
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


def move_left(state):
    # Move all tiles to the left
    new_state = 0
    for i in range(4):
        row = [get_tile(state, i * 4 + j) for j in range(4)]
        row = merge(row)
        for j in range(4):
            new_state = set_tile(new_state, i * 4 + j, row[j])
    return new_state

def move_right(state):
    # Move all tiles to the right
    new_state = 0
    for i in range(4):
        row = [get_tile(state, i * 4 + j) for j in range(4)]
        row = merge(row, reverse=True)
        for j in range(4):
            new_state = set_tile(new_state, i * 4 + j, row[j])
    return new_state


def move_up(state):
    # Move all tiles up
    new_state = 0
    for j in range(4):
        col = [get_tile(state, i * 4 + j) for i in range(4)]
        col = merge(col)
        for i in range(4):
            new_state = set_tile(new_state, i * 4 + j, col[i])
    return new_state


def move_down(state):
    # Move all tiles down
    new_state = 0
    for j in range(4):
        col = [get_tile(state, i * 4 + j) for i in range(4)]
        col = merge(col, reverse=True)
        for i in range(4):
            new_state = set_tile(new_state, i * 4 + j, col[i])
    return new_state


def get_possible_moves(state):
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

def make_move(state, move):
    # Make a move
    if move == 'left':
        state = move_left(state)
        state = add_random_tile(state)
    elif move == 'right':
        state = move_right(state)
        state = add_random_tile(state)
    elif move == 'up':
        state = move_up(state)
        state = add_random_tile(state)
    elif move == 'down':
        state = move_down(state)
        state = add_random_tile(state)
    return state


def is_game_over(state):
    # Return True if the game is over
    return not get_possible_moves(state)


def print_board(state):
    # Print the board
    for i in range(4):
        for j in range(4):
            tile = get_tile(state, i * 4 + j)
            print(2 ** tile if tile else '.', end='\t')
        print()
    print()
