import queue
from queue import Queue

from StateNode import StateNode


class StateTree:
    root: StateNode
    visited: set
    solution: StateNode
    node_queue: Queue
    parents: dict

    def __init__(self, root, goal):
        self.root = root
        self.visited = set()
        self.solution = goal
        self.node_queue = Queue()
        self.parents = dict()

    def reconstruct_path(self, state: StateNode):
        path = [state]
        parent = self.parents.get(state)
        while parent:
            path.append(parent)
            parent = self.parents.get(parent)
        path.reverse()
        return path

    def solve(self):
        self.visited.add(self.root)
        for state in self.root.generate_valid_states():
            self.node_queue.put(state)
            self.visited.add(state)
            self.parents[state] = self.root
        while self.solution not in self.visited:
            try:
                next_state = self.node_queue.get_nowait()
            except queue.Empty:
                print('Unsolvable')
                return None
            for state in next_state.generate_valid_states():
                if state not in self.visited:
                    self.visited.add(state)
                    self.node_queue.put(state)
                    self.parents[state] = next_state

        return self.reconstruct_path(self.solution)