impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut freq = [0; 26];

        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        let mut first = String::new();
        let mut middle = String::new();

        for i in 0..26 {
            for _ in 0..(freq[i] / 2) {
                first.push((b'a' + i as u8) as char);
            }
            if freq[i] % 2 == 1 {
                middle.push((b'a' + i as u8) as char);
            }
        }

        let second: String = first.chars().rev().collect();

        first + &middle + &second
    }
}