impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
         let n = nums.len();
         let mut j = -1;
         for i in 0..n {
            if nums[i] == 0 {
                j = i as i32;
                break;
            }
        }
         if j == -1 {
            return;
        }
        for i in (j + 1) as usize..n {
            if nums[i] != 0 {
                nums.swap(i, j as usize);
                j += 1;
            }
        }
    }
}