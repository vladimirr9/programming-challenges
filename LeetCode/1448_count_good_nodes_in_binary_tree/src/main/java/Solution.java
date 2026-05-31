import java.util.List;

class Solution {
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode() {}
        TreeNode(int val) { this.val = val; }
        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    int goodNodes = 1;


    public int goodNodes(TreeNode root) {
        crawl(root.left, root.val);
        crawl(root.right, root.val);
        return goodNodes;
    }

    private void crawl(TreeNode node, int maxEncountered) {
        if (node == null) {
            return;
        }
        if (node.val >= maxEncountered) {
            goodNodes++;
        }
        int newMaxEncountered = Math.max(node.val, maxEncountered);
        crawl(node.left, newMaxEncountered);
        crawl(node.right, newMaxEncountered);

    }
}