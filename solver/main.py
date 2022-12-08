from algorithms.base import Algo
from algorithms.random import RandomAlgo

from game import get_initial_state, print_board, is_game_over, make_move


def simulate(algo: Algo):
    state = get_initial_state()
    print_board(state)

    while not is_game_over(state):
        move = algo.get_move(state)
        print('Move:', move)
        state = make_move(state, move)
        print_board(state)

    print('Game over!')


simulate(RandomAlgo())