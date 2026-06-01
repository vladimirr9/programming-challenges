import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    private final Solution solution = new Solution();

    @Test
    void example1() {
        int[][] grid = {
                {2, 1, 1},
                {1, 1, 0},
                {0, 1, 1}
        };

        assertEquals(4, solution.orangesRotting(grid));
    }

    @Test
    void example2() {
        int[][] grid = {
                {2, 1, 1},
                {0, 1, 1},
                {1, 0, 1}
        };

        assertEquals(-1, solution.orangesRotting(grid));
    }

    @Test
    void example3() {
        int[][] grid = {
                {0, 2}
        };

        assertEquals(0, solution.orangesRotting(grid));
    }

}