impl Solution {
    pub fn make_fancy_string(s: String) -> String {
         let mut cnt = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut result = Vec::new();

        for i in 0..chars.len() {
            cnt = if i >= 1 && chars[i] == chars[i - 1] { cnt + 1 } else { 1 };
             if cnt < 3 {
                result.push(chars[i]);
            }
        }

        result.into_iter().collect()
    }
}