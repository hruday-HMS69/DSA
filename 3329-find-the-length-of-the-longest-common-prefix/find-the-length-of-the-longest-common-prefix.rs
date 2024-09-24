use std::collections::HashSet;
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut s: HashSet<i32> = HashSet::new();

        for mut x in arr1 {
            while x > 0 {
                s.insert(x);
                x /= 10;
            }
        }

        let mut ans = 0;

        for mut x in arr2 {
            while x > 0 {
                if s.contains(&x) {
                    ans = ans.max((x as f64).log10().floor() as i32 + 1);
                    break;
                }
                x /= 10;
            }
        }

        ans
    }
}
