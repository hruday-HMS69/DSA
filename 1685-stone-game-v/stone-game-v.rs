impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();

        let mut prefix = vec![0i32; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + stone_value[i];
        }

        let mut dp = vec![vec![0i32; n]; n];

        for len in 2..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                let mut best = 0;

                for k in l..r {
                    let left_sum = prefix[k + 1] - prefix[l];
                    let right_sum = prefix[r + 1] - prefix[k + 1];

                    let score = if left_sum < right_sum {
                        left_sum + dp[l][k]
                    } else if right_sum < left_sum {
                        right_sum + dp[k + 1][r]
                    } else {
                        left_sum + dp[l][k].max(dp[k + 1][r])
                    };

                    best = best.max(score);
                }

                dp[l][r] = best;
            }
        }

        dp[0][n - 1]
    }
}