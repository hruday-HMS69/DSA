impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![vec![None; n]; n];

        fn solve(
            l: usize,
            r: usize,
            nums: &Vec<i32>,
            dp: &mut Vec<Vec<Option<i32>>>,
        ) -> i32 {
            if l == r {
                return nums[l];
            }

            if let Some(v) = dp[l][r] {
                return v;
            }

            let take_left = nums[l] - solve(l + 1, r, nums, dp);
            let take_right = nums[r] - solve(l, r - 1, nums, dp);

            let ans = take_left.max(take_right);
            dp[l][r] = Some(ans);
            ans
        }

        solve(0, n - 1, &nums, &mut dp) >= 0
    }
}