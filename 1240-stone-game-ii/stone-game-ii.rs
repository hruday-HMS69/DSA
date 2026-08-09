impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();

        let mut suffix = vec![0; n + 1];
        for i in (0..n).rev() {
            suffix[i] = suffix[i + 1] + piles[i];
        }

        let mut dp = vec![vec![0; n + 1]; n + 1];

        for i in (0..n).rev() {
            for m in (1..=n).rev() {
                if i + 2 * m >= n {
                    dp[i][m] = suffix[i];
                    continue;
                }

                for x in 1..=2 * m {
                    let next = i + x;
                    let next_m = m.max(x);

                    dp[i][m] = dp[i][m].max(
                        suffix[i] - dp[next][next_m]
                    );
                }
            }
        }

        dp[0][1]
    }
}