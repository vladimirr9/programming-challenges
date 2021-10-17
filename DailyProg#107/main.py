from queue import Queue


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

node_queue = Queue()


def print_tree(tree: Node):
    if tree.left:
        node_queue.put(tree.left)
    if tree.right:
        node_queue.put(tree.right)
    print(tree.val)
    if not node_queue.empty():
        print_tree(node_queue.get_nowait())


print_tree(tree)
