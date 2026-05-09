import java.util.*;

class Solution {
    public int[][] rotateGrid(int[][] grid, int k) {
        int m = grid.length;
        int n = grid[0].length;

        int top = 0, bottom = m - 1;
        int left = 0, right = n - 1;

        while (top < bottom && left < right) {
            List<Integer> layer = new ArrayList<>();

            for (int j = left; j < right; j++) {
                layer.add(grid[top][j]);
            }

            for (int i = top; i < bottom; i++) {
                layer.add(grid[i][right]);
            }

            for (int j = right; j > left; j--) {
                layer.add(grid[bottom][j]);
            }

            for (int i = bottom; i > top; i--) {
                layer.add(grid[i][left]);
            }

            int size = layer.size();
            int rot = k % size;

            List<Integer> rotated = new ArrayList<>();

            for (int i = 0; i < size; i++) {
                rotated.add(layer.get((i + rot) % size));
            }

            int idx = 0;

            for (int j = left; j < right; j++) {
                grid[top][j] = rotated.get(idx++);
            }

            for (int i = top; i < bottom; i++) {
                grid[i][right] = rotated.get(idx++);
            }

            for (int j = right; j > left; j--) {
                grid[bottom][j] = rotated.get(idx++);
            }

            for (int i = bottom; i > top; i--) {
                grid[i][left] = rotated.get(idx++);
            }

            top++;
            bottom--;
            left++;
            right--;
        }

        return grid;
    }
}