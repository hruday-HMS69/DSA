impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mx = *candidates.iter().max().unwrap();
        let m = 32 - mx.leading_zeros();
        let mut ans = 0;

        for i in 0..m {
            let mut cnt = 0;
            for &x in &candidates {
                cnt += (x >> i) & 1;
            }
            ans = ans.max(cnt);
        }
        ans
    }
}