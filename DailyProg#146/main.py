class Node:
    def __init__(self, data):
        self.left = None
        self.right = None
        self.data = data
        self.status = True

    def printNode(self):
        if self is not None:
            print(self.data)

    def printTree(self):
        if self is not None:
            if self.left:
                self.left.printTree()
            if self.data is not None:
                print(self.data)
            if self.right:
                self.right.printTree()

    def insert(self, data):
        if self.left is None:
            self.left = Node(data)
        elif self.right is None:
            self.right = Node(data)
        elif self.status:
            self.left.insert(data)
            self.status = not self.status
        else:
            self.right.insert(data)
            self.status = not self.status

    def deleteNode(self):
        self.data = None
        if self.left is not None:
            self.left.deleteNode()
        if self.right is not None:
            self.right.deleteNode()
        self.left = None
        self.right = None
        self = None

    def removeLoose(self):
        val = self.data
        if self is None:
            return 0
        if self.left is not None:
            val += self.left.removeLoose()
        if self.right is not None:
            val += self.right.removeLoose()
        if val == 0:
            self.deleteNode()
        return val



root = Node(1)
root.insert(0)
root.insert(0)
root.insert(0)
root.insert(0)
root.insert(0)
root.insert(0)
root.insert(0)
root.insert(1)


#root.printTree()
root.removeLoose()
root.printTree()
