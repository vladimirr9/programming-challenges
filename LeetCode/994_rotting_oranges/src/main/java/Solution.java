import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

class Solution {
    public int orangesRotting(int[][] grid) {
        int fresh = 0;
        List<Pair> rottens = new ArrayList<>();

        for (int i = 0; i < grid.length; i++) {
            for (int j = 0; j < grid[i].length; j++) {
                if (grid[i][j] == 1) {
                    fresh++;
                } else if (grid[i][j] == 2) {
                    rottens.add(new Pair(i, j));
                }
            }
        }
        if (fresh == 0) {
            return 0;
        }
        if (rottens.isEmpty()) {
            return -1;
        }
        boolean changes;
        int iterations = 0;
        do {
            Set<Pair> freshToRotten = new HashSet<>();

            for (var rotten : rottens) {
                List<Pair> freshes = getSurroundingFreshes(grid, rotten.x, rotten.y);
                freshToRotten.addAll(freshes);
            }
            changes = !freshToRotten.isEmpty();
            for (var freshOrange : freshToRotten) {
                grid[freshOrange.x][freshOrange.y] = 2;
                fresh--;
            }
            rottens.addAll(freshToRotten);
            if (changes) {
                iterations++;
            }
        } while (changes);
        if (fresh > 0) {
            return -1;
        }
        return iterations;
    }

    int[][] directions = {
            {-1, 0}, // up
            {1, 0},  // down
            {0, -1}, // left
            {0, 1}   // right
    };

    private List<Pair> getSurroundingFreshes(int[][] grid, int x, int y) {
        int m = grid.length;
        int n = grid[0].length;
        List<Pair> freshes = new ArrayList<>();

        for (int[] dir : directions) {
            int newX = x + dir[0];
            int newY = y + dir[1];

            if (newX >= 0 && newX < m && newY >= 0 && newY < n) {
                if (grid[newX][newY] == 1) {
                    freshes.add(new Pair(newX, newY));
                }
            }
        }
        return freshes;
    }

    public record Pair(int x, int y) {
    }
}