package org.example;

import org.junit.jupiter.api.Test;


import static org.example.Solution.*;
import static org.junit.jupiter.api.Assertions.*;

class MainTest {

    @Test
    void testCase1() {
        var solution = new Solution();
        var root = new TreeNode(1, new TreeNode(2, new TreeNode(4), new TreeNode(5)), new TreeNode(3));
        assertEquals(3, solution.height(root));
        assertEquals(3, solution.diameterOfBinaryTree(root));
    }

    @Test
    void testCase2() {
        var solution = new Solution();
        var root = new TreeNode(1, new TreeNode(2), null);
        assertEquals(2, solution.height(root));
        assertEquals(1, solution.diameterOfBinaryTree(root));
    }
}