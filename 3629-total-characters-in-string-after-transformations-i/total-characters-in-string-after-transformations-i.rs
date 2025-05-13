impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: usize = 1_000_000_007;
        let t = t as usize;

        let mut dp = vec![vec![0usize; t + 1]; 26];

        for c in 0..26 {
            dp[c][0] = 1;
        }

        for i in 1..=t {
            for c in 0..26 {
                if c == 25 {
                   
                    dp[c][i] = (dp[0][i - 1] + dp[1][i - 1]) % MOD;
                } else {
                    dp[c][i] = dp[c + 1][i - 1];
                }
            }
        }
        let mut result: usize = 0;
        for ch in s.chars() {
            let idx = (ch as u8 - b'a') as usize;
            result = (result + dp[idx][t]) % MOD;
        }

        result as i32
    }
}
