from StateNode import StateNode
from StateTree import StateTree

#root = StateNode([[8, 3, 5], [4, 1, 6], [2, 7, None]])
#root = StateNode([[1, 2, 3], [4, 5, 6], [7, None, 8]])
root = StateNode([[1, 8, 2], [None, 4, 3], [7, 6, 5]])

goal = StateNode([[1, 2, 3], [4, 5, 6], [7, 8, None]])
tree = StateTree(root, goal)
solution = tree.solve()
if solution:
    for state in solution:
        print(state)
        print('---')

