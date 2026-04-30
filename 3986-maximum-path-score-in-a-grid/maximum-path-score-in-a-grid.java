class Solution {
    public int maxPathScore(int[][] grid, int k) {
        int m = grid.length, n = grid[0].length;

        int[][][] dp = new int[m][n][k + 1];

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                for (int c = 0; c <= k; c++) {
                    dp[i][j][c] = -1;
                }
            }
        }

        dp[0][0][0] = 0;

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {

                for (int c = 0; c <= k; c++) {
                    if (dp[i][j][c] == -1) continue;

                    if (j + 1 < n) {
                        int nextVal = grid[i][j + 1];
                        int nextCost = (nextVal == 0) ? 0 : 1;

                        if (c + nextCost <= k) {
                            dp[i][j + 1][c + nextCost] =
                                Math.max(dp[i][j + 1][c + nextCost],
                                         dp[i][j][c] + nextVal);
                        }
                    }

                    if (i + 1 < m) {
                        int nextVal = grid[i + 1][j];
                        int nextCost = (nextVal == 0) ? 0 : 1;

                        if (c + nextCost <= k) {
                            dp[i + 1][j][c + nextCost] =
                                Math.max(dp[i + 1][j][c + nextCost],
                                         dp[i][j][c] + nextVal);
                        }
                    }
                }
            }
        }

        int ans = -1;
        for (int c = 0; c <= k; c++) {
            ans = Math.max(ans, dp[m - 1][n - 1][c]);
        }

        return ans;
    }
}