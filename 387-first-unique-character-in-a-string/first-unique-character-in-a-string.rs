use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut freq= [0; 26];
        
        for ch in s.chars(){
            let idx = (ch as u8 - b'a') as usize;
            freq[idx] += 1;
        }
        for (i,ch) in s.chars().enumerate(){
            let idx = (ch as u8 - b'a') as usize;
            if freq[idx] == 1 {
                return i as i32;
            }
        }
        -1
    }
}