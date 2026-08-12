use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = HashMap::new();
        let mut left = 0;
        let mut ans = 0;

        for right in 0..nums.len() {
            *freq.entry(nums[right]).or_insert(0) += 1;

            while freq[&nums[right]] > k {
                freq.entry(nums[left])
                    .and_modify(|count| *count -= 1);

                left += 1;
            }

            ans = ans.max((right - left + 1) as i32);
        }

        ans
    }
}