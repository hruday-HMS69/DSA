impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();

        let mut suffix = vec![0; n + 1];
        for i in (0..n).rev() {
            suffix[i] = suffix[i + 1] + piles[i];
        }

        let mut dp = vec![vec![-1; n + 1]; n + 1];

        fn solve(
            i: usize,
            m: usize,
            n: usize,
            suffix: &Vec<i32>,
            dp: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i >= n {
                return 0;
            }

            if dp[i][m] != -1 {
                return dp[i][m];
            }

            if i + 2 * m >= n {
                dp[i][m] = suffix[i];
                return suffix[i];
            }

            let mut best = 0;

            for x in 1..=2 * m {
                if i + x > n {
                    break;
                }

                let opponent = solve(
                    i + x,
                    m.max(x),
                    n,
                    suffix,
                    dp,
                );

                let current = suffix[i] - opponent;
                best = best.max(current);
            }

            dp[i][m] = best;
            best
        }

        solve(0, 1, n, &suffix, &mut dp)
    }
}