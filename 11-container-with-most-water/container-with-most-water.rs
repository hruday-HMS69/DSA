impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp;
        let (mut l, mut r) = (0, height.len() - 1);
        let mut area = 0;
        while l < r {
            area = cmp::max(area, cmp::min(height[l], height[r]) * (r - l) as i32);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        area
    }
}