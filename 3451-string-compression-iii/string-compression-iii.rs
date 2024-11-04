impl Solution {
    pub fn compressed_string(word: String) -> String {
         let mut ans = String::new();
        let n = word.len();
        let chars: Vec<char> = word.chars().collect(); 

        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && chars[j] == chars[i] {
                j += 1;
            }
            let mut k = j - i;
            while k > 0 {
                let x = k.min(9);
                ans.push((x as u8 + b'0') as char); 
                ans.push(chars[i]);
                k -= x;
            }
            i = j;
        }

        ans
    }
}