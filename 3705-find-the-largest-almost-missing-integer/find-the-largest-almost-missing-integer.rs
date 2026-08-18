impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut count = vec![0; 51];

        for i in 0..=n - k {
            let mut seen = vec![false; 51];

            for j in i..i + k {
                seen[nums[j] as usize] = true;
            }

            for x in 0..=50 {
                if seen[x] {
                    count[x] += 1;
                }
            }
        }

        for x in (0..=50).rev() {
            if count[x] == 1 {
                return x as i32;
            }
        }

        -1
    }
}