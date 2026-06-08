import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

class SolutionTest {

  private final Solution solution = new Solution();

  @Test
  void rotates3x3MatrixInPlace() {
	int[][] matrix = {
	  {1, 2, 3},
	  {4, 5, 6},
	  {7, 8, 9}
	};
	int[][] expected = {
	  {7, 4, 1},
	  {8, 5, 2},
	  {9, 6, 3}
	};

		assertRotatedInPlace(matrix, expected);
  }

  @Test
  void rotates4x4MatrixInPlace() {
	int[][] matrix = {
	  {5, 1, 9, 11},
	  {2, 4, 8, 10},
	  {13, 3, 6, 7},
	  {15, 14, 12, 16}
	};
	int[][] expected = {
	  {15, 13, 2, 5},
	  {14, 3, 4, 1},
	  {12, 6, 8, 9},
	  {16, 7, 10, 11}
	};

	assertRotatedInPlace(matrix, expected);
  }

  @Test
  void rotates6x6MatrixInPlace() {
	int[][] matrix = {
	  {2, 29, 20, 26, 16, 28},
	  {12, 27, 9, 25, 13, 21},
	  {32, 33, 32, 2, 28, 14},
	  {13, 14, 32, 27, 22, 26},
	  {33, 1, 20, 7, 21, 7},
	  {4, 24, 1, 6, 32, 34}
	};
	int[][] expected = {
	  {4, 33, 13, 32, 12, 2},
	  {24, 1, 14, 33, 27, 29},
	  {1, 20, 32, 32, 9, 20},
	  {6, 7, 27, 2, 25, 26},
	  {32, 21, 22, 28, 13, 16},
	  {34, 7, 26, 14, 21, 28}
	};

	assertRotatedInPlace(matrix, expected);
  }

  private void assertRotatedInPlace(int[][] matrix, int[][] expected) {
	int[][] originalRows = matrix.clone();

	solution.rotate(matrix);

	for (int i = 0; i < matrix.length; i++) {
	  assertSame(originalRows[i], matrix[i], "Rotation should not replace row arrays");
	}
	assertArrayEquals(expected, matrix);
  }

}