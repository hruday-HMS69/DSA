use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut freq: HashMap<char,i32> = HashMap::new();
        
        for ch in s.chars(){
            *freq.entry(ch).or_insert(0) += 1;
        }
        for (i,ch) in s.chars().enumerate(){
            if freq[&ch]==1{
                return i as i32;
            }
        }
        -1
    }
}