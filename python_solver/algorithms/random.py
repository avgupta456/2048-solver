import random

from algorithms.base import Algo
from game import get_possible_moves


class RandomAlgo(Algo):
    def get_move(self, state):
        # Make a random move
        moves = get_possible_moves(state)
        return random.choice(moves)
