from datetime import datetime, timedelta

from algorithms.base import Algo
from algorithms.random import RandomAlgo
from algorithms.expectimax import ExpectimaxAlgo

from game import get_initial_state, get_score, is_game_over, make_move


def run_game(algo: Algo):
    moves = 0
    state = get_initial_state()
    while not is_game_over(state):
        move = algo.get_move(state)
        state = make_move(state, move)
        moves += 1
    return get_score(state), moves

def simulate(algo: Algo):
    start = datetime.now()
    time = 10

    total_score = 0
    total_games = 0
    total_moves = 0

    while datetime.now() - start < timedelta(seconds=time):
        score, moves = run_game(algo)
        total_score += score
        total_moves += moves
        total_games += 1

    print("Games / Sec", total_games / time)
    print("Moves / Sec", total_moves / time)
    print("Score / Game", total_score / total_games)

simulate(RandomAlgo())