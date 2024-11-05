impl Solution {
    pub fn min_changes(s: String) -> i32 {
         let mut ans = 0;
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();

        for i in (1..n).step_by(2) {
            if chars[i] != chars[i - 1] {
                ans += 1;
            }
        }

        ans
    }
}