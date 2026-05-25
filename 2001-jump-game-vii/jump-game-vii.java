class Solution {
    public boolean canReach(String s, int minJump, int maxJump) {
        int n = s.length();
        boolean[] dp = new boolean[n];
        dp[0] = true;
        
        int[] prefixSum = new int[n]; 
        prefixSum[0] = 1; 
        
        for (int i = 1; i < n; i++) {
            if (s.charAt(i) == '0') {
                int left = Math.max(0, i - maxJump);
                int right = i - minJump;
                
                if (right >= 0) {
                    int reachableCount = prefixSum[right] - (left > 0 ? prefixSum[left - 1] : 0);
                    if (reachableCount > 0) {
                        dp[i] = true;
                    }
                }
            }
            
            prefixSum[i] = prefixSum[i - 1] + (dp[i] ? 1 : 0);
        }
        
        return dp[n - 1];
    }
}