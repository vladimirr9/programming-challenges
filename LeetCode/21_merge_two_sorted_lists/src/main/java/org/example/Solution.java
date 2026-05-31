package org.example;



class Solution {
    public static class ListNode {
        int val;
        ListNode next;
        ListNode() {}
        ListNode(int val) { this.val = val; }
        ListNode(int val, ListNode next) { this.val = val; this.next = next; }
    }


    static void main() {

    }

    public ListNode mergeTwoLists(ListNode list1, ListNode list2) {
        ListNode root = null;
        if (list1 == null && list2 == null) {
            return root;
        } else if (list1 == null) {
            root = new ListNode(list2.val);
            root.next = mergeTwoLists(list1, list2.next);
        } else if (list2 == null) {
            root = new ListNode(list1.val);
            root.next = mergeTwoLists(list1.next, list2);
        } else {
            if (list1.val > list2.val) {
                root = new ListNode(list2.val);
                root.next = mergeTwoLists(list1, list2.next);
            } else {
                root = new ListNode(list1.val);
                root.next = mergeTwoLists(list1.next, list2);
            }
        }
        return root;
    }



}