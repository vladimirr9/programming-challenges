import copy


class StateNode:
    state: list
    width = 3
    height = 3

    def __init__(self, state):
        self.state = state

    def find_empty(self) -> (int, int):
        for i in range(self.width):
            for j in range(self.height):
                if not self.state[i][j]:
                    return i, j

    def is_within_bounds(self, i: int, j: int):
        return 0 <= i < 3 and 0 <= j < 3

    def generate_valid_states(self) -> list:
        valid_states = []
        none_i, none_j = self.find_empty()
        for i, j in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            new_i = none_i + i
            new_j = none_j + j
            if self.is_within_bounds(new_i, new_j):
                new_state = copy.deepcopy(self.state)
                tmp = new_state[new_i][new_j]
                new_state[new_i][new_j] = None
                new_state[none_i][none_j] = tmp
                valid_states.append(StateNode(new_state))
        return valid_states

    def __hash__(self):
        flat = []
        for i in range(self.width):
            for j in range(self.height):
                if not self.state[i][j]:
                    flat.append('0')
                else:
                    flat.append(str(self.state[i][j]))
        return hash(tuple(flat))

    def __eq__(self, other):
        return self.__hash__() == other.__hash__()

    #str for debugging
    def __str__(self):
        flat = []
        for i in range(self.width):
            for j in range(self.height):
                if not self.state[i][j]:
                    flat.append('0')
                else:
                    flat.append(str(self.state[i][j]))
        return flat[0] + flat[1] + flat[2] + '\n' + flat[3] + flat[4] + flat[5] + '\n' + flat[6] + flat[7] + flat[8]



