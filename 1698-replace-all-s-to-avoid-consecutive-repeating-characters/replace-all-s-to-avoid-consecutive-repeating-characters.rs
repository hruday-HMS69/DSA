impl Solution {
    pub fn modify_string(s: String) -> String {
         let mut s = s.into_bytes();
    let n = s.len();
    for i in 0..n {
        if s[i] != b'?' {
            continue;
        }
        for letter in b'a'..=b'z' {
            if let Some(left) = s.get(i.wrapping_sub(1)) {
                if letter.eq(left) {
                    continue;
                }
            }
            if let Some(right) = s.get(i + 1) {
                if letter.eq(right) {
                    continue;
                }
            }
            s[i] = letter;
        }
    }
    unsafe { String::from_utf8_unchecked(s) }
    }
}