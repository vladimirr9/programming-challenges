import java.util.HashMap;
import java.util.Map;

class Solution {

  // n - 1 - x -> y
  // y -> x,
  // (0,1) -> (1,2)
  // (2,1) -> (1,0)
  // (1,1) -> (1,1)
  // (2,0) -> (0,0)
  // (0,2) -> (2,2)
  // (0,0) -> (0,2)
  public void rotate(int[][] matrix) {
    int n = matrix.length;
    Map<Pair, Integer> moveMap = new HashMap<>();
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < n; j++) {
        Pair cell = new Pair(i,j);
        Pair pair = getPair(cell, n);
        moveMap.put(pair, matrix[i][j]);
      }
    }
    for (var move : moveMap.entrySet()) {
      matrix[move.getKey().x()][move.getKey().y] = move.getValue();
    }

  }

  private Pair getPair(Pair cell, int n) {
    return new Pair(cell.y, n - 1 - cell.x);
  }
  record Pair(int x, int y) { }



}