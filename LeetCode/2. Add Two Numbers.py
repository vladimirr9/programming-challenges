# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

from typing import Optional



def formLinkedList(list):
    if len(list) == 0:
        return None
    head = ListNode(list[0], None)
    prev = head
    for idx in range(1,len(list)):
        node = ListNode(list[idx], None)
        prev.next = node
        prev = node
    return head

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        suma = []
        carry = 0
        while l1 or l2:
            val_1 = 0
            val_2 = 0
            if l1:
                val_1 = l1.val
                l1 = l1.next
            if l2:
                val_2 = l2.val
                l2 = l2.next
            digit = (val_1 + val_2 + carry)
            suma.append(digit)
            carry = 1 if (val_1 + val_2 + carry) % 10 > 0 else 0
        if carry:
            suma.append(carry)
        print(suma)
        pass

if __name__ == '__main__':
    l1 = [2, 4, 3]
    l2 = [5, 6, 4]
    ll1 = formLinkedList(l1)
    ll2 = formLinkedList(l2)
    sol = Solution()
    print(sol.addTwoNumbers(ll1, ll2))