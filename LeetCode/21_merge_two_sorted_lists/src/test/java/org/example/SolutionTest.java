package org.example;

import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.example.Solution.*;
import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    @Test
    public void testMergeTwoLists1() {
        ListNode list1 = createList(1, 2, 4);
        ListNode list2 = createList(1, 3, 4);

        ListNode result = new Solution().mergeTwoLists(list1, list2);

        assertArrayEquals(
                new int[]{1, 1, 2, 3, 4, 4},
                toArray(result)
        );
    }

    @Test
    public void testMergeTwoLists2() {
        ListNode list1 = null;
        ListNode list2 = null;

        ListNode result = new Solution().mergeTwoLists(list1, list2);

        assertArrayEquals(
                new int[]{},
                toArray(result)
        );
    }

    @Test
    public void testMergeTwoLists3() {
        ListNode list1 = null;
        ListNode list2 = createList(0);

        ListNode result = new Solution().mergeTwoLists(list1, list2);

        assertArrayEquals(
                new int[]{0},
                toArray(result)
        );
    }

    private ListNode createList(int... values) {
        if (values == null || values.length == 0) {
            return null;
        }

        ListNode dummy = new ListNode(0);
        ListNode current = dummy;

        for (int value : values) {
            current.next = new ListNode(value);
            current = current.next;
        }

        return dummy.next;
    }

    private int[] toArray(ListNode head) {
        List<Integer> values = new ArrayList<>();

        while (head != null) {
            values.add(head.val);
            head = head.next;
        }

        return values.stream()
                .mapToInt(Integer::intValue)
                .toArray();
    }
}