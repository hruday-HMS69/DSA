impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut l = 0;
        let mut r = s.chars().filter(|&c| c == '1').count() as i32;
        let mut ans = 0;
        
        for i in 0..s.len() - 1 {
            if s.chars().nth(i).unwrap() == '0' {
                l += 1;
            } else {
                r -= 1;
            }
            ans = ans.max(l + r);
        }
        
        ans
    }
}
