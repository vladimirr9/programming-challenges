import Solution.*

fun main() {
    val root = TreeNode(4)
    root.left = TreeNode(2)
    root.left!!.left = TreeNode(1)
    root.left!!.right = TreeNode(3)
    root.right = TreeNode(7)
    root.right!!.left = TreeNode(6)
    root.right!!.right = TreeNode(9)
    val solution = Solution()
    println(solution.getTreeAsString(root))
    println(solution.getTreeAsString(solution.invertTree(root)))
}


class Solution {
    fun invertTree(root: TreeNode?): TreeNode? {
        if (root == null) {
            return null
        }
        invertTree(root.left)
        invertTree(root.right)
        val newLeft = root.right
        val newRight = root.left
        root.left = newLeft
        root.right = newRight
        return root
    }


    fun getTreeAsString(root: TreeNode?): String {
        val vals: MutableList<Int> = mutableListOf()
        if (root == null) {
            return vals.toString()
        }
        val queue: MutableList<TreeNode> = mutableListOf(root)
        while (queue.isNotEmpty()) {
            val pointer = queue.removeFirst()!!
            vals.add(pointer.value)
            if (pointer.left != null) {
                queue.add(pointer.left!!);
            }
            if (pointer.right != null) {
                queue.add(pointer.right!!);
            }
        }
        return vals.toString()
    }

    data class TreeNode(val value: Int) {
        var left: TreeNode? = null
        var right: TreeNode? = null
    }
}


