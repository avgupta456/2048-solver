from functools import lru_cache

from algorithms.base import Algo
from game import get_grid, make_move, get_empty_tiles, get_possible_moves, set_tile, get_score

class ExpectimaxAlgo(Algo):
    @lru_cache()
    def heuristic(self, state):
        grid = get_grid(state, exp=True)
        lower_left = 10 * grid[3][0] + 5 * grid[2][0] + 2 * grid[1][0] + grid[0][0] \
                    + 5 * grid[3][1] + 2 * grid[2][1] + grid[1][1] \
                    + 2 * grid[3][2] + grid[2][2] \
                    + grid[3][3]
        
        lower_right = 10 * grid[3][3] + 5 * grid[2][3] + 2 * grid[1][3] + grid[0][3] \
                    + 5 * grid[3][2] + 2 * grid[2][2] + grid[1][2] \
                    + 2 * grid[3][1] + grid[2][1] \
                    + grid[3][0]

        upper_left = 10 * grid[0][0] + 5 * grid[1][0] + 2 * grid[2][0] + grid[3][0] \
                    + 5 * grid[0][1] + 2 * grid[1][1] + grid[2][1] \
                    + 2 * grid[0][2] + grid[1][2] \
                    + grid[0][3]

        upper_right = 10 * grid[0][3] + 5 * grid[1][3] + 2 * grid[2][3] + grid[3][3] \
                    + 5 * grid[0][2] + 2 * grid[1][2] + grid[2][2] \
                    + 2 * grid[0][1] + grid[1][1] \
                    + grid[0][0]

        return max(lower_left, lower_right, upper_left, upper_right)


    def _get_move(self, state, depth):
        if depth == 0:
            return None, self.heuristic(state)

        moves = get_possible_moves(state)
        if not moves:
            return None, 0, []

        best_move = None
        best_score = float('-inf')
        move_scores = []
        for move in moves:
            temp_state = make_move(state, move, add_tile=False)
            empty_tiles = get_empty_tiles(temp_state)
            score = 0
            # print("TILES", empty_tiles)
            for tile in empty_tiles:
                temp_state = set_tile(temp_state, tile, 1)
                score += 0.9 * self._get_move(temp_state, depth - 1)[1]
                temp_state = set_tile(temp_state, tile, 2)
                score += 0.1 * self._get_move(temp_state, depth - 1)[1]
            score /= len(empty_tiles)
            if score > best_score:
                best_score = score
                best_move = move
            move_scores.append((move, score))
        return best_move, best_score, move_scores

    def get_move(self, state):
        curr_score = self.heuristic(state)
        move, orig_best_score, move_scores = self._get_move(state, 2)
        if len(move_scores) == 1:
            return move

        print(curr_score, move_scores)
        second_best_score = sorted(move_scores, key=lambda x: x[1], reverse=True)[1][1]
        best_score = max(1, orig_best_score - curr_score)
        second_best_score = max(1, second_best_score - curr_score)
        if orig_best_score > 2000 and second_best_score / best_score > 0.75:
            move, best_score, move_scores = self._get_move(state, 3)
        return move
        
