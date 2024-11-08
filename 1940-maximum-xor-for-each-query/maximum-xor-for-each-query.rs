impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
          let mut xs = 0;
        for &x in &nums {
            xs ^= x;
        }

        let n = nums.len();
        let mut ans = vec![0; n];

        for i in 0..n {
            let x = nums[n - i - 1];
            let mut k = 0;

            for j in (0..maximum_bit).rev() {
                if (xs >> j) & 1 == 0 {
                    k |= 1 << j;
                }
            }

            ans[i] = k;
            xs ^= x;
        }

        ans
    }
}