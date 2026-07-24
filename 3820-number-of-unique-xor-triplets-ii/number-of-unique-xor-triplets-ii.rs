use std::collections::HashSet;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 1 {
            return 1;
        }

        let mut pairs = HashSet::new();

        for i in 0..n {
            for j in i + 1..n {
                pairs.insert(nums[i] ^ nums[j]);
            }
        }

        let mut triplets = HashSet::new();

        for &pair in &pairs {
            for &num in &nums {
                triplets.insert(pair ^ num);
            }
        }

        triplets.len() as i32
    }
}
