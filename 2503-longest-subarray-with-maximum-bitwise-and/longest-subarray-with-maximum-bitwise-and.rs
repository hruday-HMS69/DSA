impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
         let max_num = *nums.iter().max().unwrap();
        let mut cnt = 0;
        let mut ans = 0;
        for num in nums {
            if num == max_num {
                cnt += 1;
            } else {
                ans = ans.max(cnt);
                cnt = 0;
            }
        }
        return ans.max(cnt)
    }
}