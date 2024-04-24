fun main() {
    val l1 = getListNodes(intArrayOf(2,4,3))
    val l2 = getListNodes(intArrayOf(5,6,4))
    print(addTwoNumbers(l1, l2)!!)
}

/**
 * Example:
 * var li = ListNode(5)
 * var v = li.`val`
 * Definition for singly-linked list.

 */
fun addTwoNumbers(l1: ListNode?, l2: ListNode?): ListNode? {
    if (l1 == null || l2 == null) {
        return null
    }
    var left = l1
    var right = l2
    val root = ListNode(0)
    var pointer = root
    var carry = 0
    while (true) {
        val val1 = left?.`val` ?: 0
        val val2 = right?.`val` ?: 0
        val sum = val1 + val2 + carry
        val nodeValue = sum % 10
        carry = sum / 10
        pointer.`val` = nodeValue
        left = left?.next
        right = right?.next
        if (left == null && right == null && carry == 0) {
            return root
        } else {
            pointer.next = ListNode(0)
            pointer = pointer.next!!
        }
    }
}

class ListNode(var `val`: Int) {
    var next: ListNode? = null
}



fun getListNodes(arr: IntArray) : ListNode {
    val root = ListNode(arr.first())
    var pointer = root;
    for (i in 1..<arr.size) {
        pointer.next = ListNode(arr[i])
        pointer = pointer.next!!
    }
    return root
}

fun print(listNode: ListNode) {
    print("${listNode.`val`}, ")
    var pointer = listNode.next
    while (pointer != null) {
        print("${pointer.`val`}, ")
        pointer = pointer.next
    }
    println()
}