impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut f = vec![-2; 32];
        f[0] = -1;
        let vowels = "aeiou".chars().collect::<Vec<_>>();
        let mut ans = 0;
        let mut now = 0;

        for (i, ch) in s.chars().enumerate() {
            if let Some(pos) = vowels.iter().position(|&v| v == ch) {
                now ^= 1 << pos;
            }

            if f[now] != -2 {
                ans = ans.max((i as i32) - f[now]);
            } else {
                f[now] = i as i32;
            }
        }

        ans
    }
}