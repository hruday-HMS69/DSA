use std::collections::HashSet;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
        let mut nums = Vec::new();

        for (i, word) in words.iter().enumerate() {
            let a = word.chars().next().unwrap();  
            let b = word.chars().last().unwrap(); 
            if vowels.contains(&a) && vowels.contains(&b) {
                nums.push(i as i32); 
            }
        }

        let mut ans = Vec::new();
        for q in queries {
            let l = q[0];
            let r = q[1];
            
            
            let lower = nums.binary_search(&l).unwrap_or_else(|x| x);
            let upper = nums.binary_search(&(r + 1)).unwrap_or_else(|x| x);

            let count = upper - lower;
            ans.push(count as i32);
        }

        ans
    }
}
