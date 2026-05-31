import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {


    Solution solution = new Solution();

    // Helper to build tree from LeetCode-style array (level order)
    private Solution.TreeNode build(Integer[] arr) {
        return build(arr, 0);
    }

    private Solution.TreeNode build(Integer[] arr, int i) {
        if (i >= arr.length || arr[i] == null) return null;

        Solution.TreeNode node = new Solution.TreeNode(arr[i]);
        node.left = build(arr, 2 * i + 1);
        node.right = build(arr, 2 * i + 2);
        return node;
    }

    @Test
    void testExample1() {
        Solution.TreeNode root = build(new Integer[]{
                3, 1, 4, 3, null, 1, 5
        });

        assertEquals(4, solution.goodNodes(root));
    }

    @Test
    void testExample2() {
        Solution.TreeNode root = build(new Integer[]{
                3, 3, null, 4, 2
        });

        assertEquals(3, solution.goodNodes(root));
    }

    @Test
    void testExample3_singleNode() {
        Solution.TreeNode root = build(new Integer[]{
                1
        });

        assertEquals(1, solution.goodNodes(root));
    }

    @Test
    void testAllIncreasingPath() {
        Solution.TreeNode root = build(new Integer[]{
                1, 2, 3, 4, 5
        });

        assertEquals(5, solution.goodNodes(root));
    }

    @Test
    void testAllSameValues() {
        Solution.TreeNode root = build(new Integer[]{
                2, 2, 2, 2, 2
        });

        assertEquals(5, solution.goodNodes(root));
    }

    @Test
    void testNoGoodNodesExceptRoot() {
        Solution.TreeNode root = build(new Integer[]{
                5, 1, 1, 1, 1
        });

        assertEquals(1, solution.goodNodes(root));
    }
}