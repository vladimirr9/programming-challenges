class Node:
    def __init__(self, val: int):
        self.val = val
        self.left = None
        self.right = None


tree = Node(1)
tree.left = Node(2)
tree.right = Node(3)
tree.right.left = Node(4)
tree.right.right = Node(5)

paths = []

parents = dict()


def reconstruct_path(node):
    parent = parents.get(node)
    path = [node.val]
    while parent:
        path.append(parent.val)
        parent = parents.get(parent)
    path.reverse()
    paths.append(path)


def find_leaves(tree: Node):
    if not tree.left and not tree.right:
        reconstruct_path(tree)
    if tree.left:
        parents[tree.left] = tree
        find_leaves(tree.left)
    if tree.right:
        parents[tree.right] = tree
        find_leaves(tree.right)

find_leaves(tree)
print(paths)