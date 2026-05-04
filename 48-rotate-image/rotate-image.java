class Solution {
    public void rotate(int[][] matrix) {
        if (matrix == null || matrix.length == 0 || matrix[0] == null || matrix[0].length == 0) { return; }
        int row = matrix.length;
        int col = matrix[0].length;
        for (int i = 0; i < row / 2; i++) {
            for (int j = 0; j < col; j++) {
                swap(matrix, i, j, row - 1 - i, j);
            }
        }
        for (int i = 0; i < row; i++) {
            for (int j = i + 1; j < col; j++) {
                swap(matrix, i, j, j, i);
            }
        }
    }
    
    private void swap(int[][] matrix, int x0, int y0, int x1, int y1) {
        int temp = matrix[x0][y0];
        matrix[x0][y0] = matrix[x1][y1];
        matrix[x1][y1] = temp;
    }
}