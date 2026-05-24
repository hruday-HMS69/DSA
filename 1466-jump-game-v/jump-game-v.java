class Solution {
    public int maxJumps(int[] arr, int d) {
        int n = arr.length;
        int[] dp = new int[n];
        int maxVisited = 0;
        
        for (int i = 0; i < n; i++) {
            maxVisited = Math.max(maxVisited, dfs(arr, d, i, dp));
        }
        
        return maxVisited;
    }
    
    private int dfs(int[] arr, int d, int index, int[] dp) {
        if (dp[index] != 0) {
            return dp[index];
        }
        
        int maxLen = 1;
        
        int right = index + 1;
        int steps = 0;
        while (right < arr.length && steps < d && arr[right] < arr[index]) {
            maxLen = Math.max(maxLen, 1 + dfs(arr, d, right, dp));
            right++;
            steps++;
        }
        
        int left = index - 1;
        steps = 0;
        while (left >= 0 && steps < d && arr[left] < arr[index]) {
            maxLen = Math.max(maxLen, 1 + dfs(arr, d, left, dp));
            left--;
            steps++;
        }
        
        dp[index] = maxLen;
        return maxLen;
    }
}