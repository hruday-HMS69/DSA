impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
          let mut pre_mx = 0;
        let mut i = 0;
        let n = nums.len();

        while i < n {
            let cnt = nums[i].count_ones(); 
            let mut j = i + 1;
            let mut mi = nums[i];
            let mut mx = nums[i];

            while j < n && nums[j].count_ones() == cnt {
                mi = mi.min(nums[j]);
                mx = mx.max(nums[j]);
                j += 1;
            }

            if pre_mx > mi {
                return false;
            }
            pre_mx = mx;
            i = j;
        }
        true
    }
}