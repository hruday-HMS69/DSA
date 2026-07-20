impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let total = m * n;
        let k = (k as usize) % total;

        let mut ans = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                let idx = i * n + j;
                let new_idx = (idx + k) % total;
                ans[new_idx / n][new_idx % n] = grid[i][j];
            }
        }

        ans
    }
}